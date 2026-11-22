use imomoe_admin_core::punishment::{Kind, Mutation, Punishment, PunishmentStore, StoreError};
use rusqlite::{Connection, Row, params};
use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

pub fn sqlite(path: &Path) -> Result<Box<dyn PunishmentStore>, StoreError> {
    create_parent(path)?;
    let conn = Connection::open(path).map_err(err)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS punishments (
           id INTEGER PRIMARY KEY, steam_id INTEGER NOT NULL, kind TEXT NOT NULL,
           actor INTEGER, reason TEXT NOT NULL, created_at INTEGER NOT NULL,
           expires_at INTEGER, revoked_at INTEGER, revoked_by INTEGER
         );
         CREATE UNIQUE INDEX IF NOT EXISTS active_punishment
           ON punishments(steam_id, kind) WHERE revoked_at IS NULL;
         CREATE INDEX IF NOT EXISTS ban_page ON punishments(kind, id)
           WHERE revoked_at IS NULL;",
    )
    .map_err(err)?;
    Ok(Box::new(Sqlite(conn)))
}

pub fn files(directory: &Path) -> Result<Box<dyn PunishmentStore>, StoreError> {
    let bans = Records::load(directory.join("bans.json"), true)?;
    let comms = Records::load(directory.join("comms.json"), false)?;
    Ok(Box::new(Files { bans, comms }))
}

fn err(error: impl std::error::Error + Send + Sync + 'static) -> StoreError {
    StoreError(Box::new(error))
}

fn create_parent(path: &Path) -> Result<(), StoreError> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(err)?;
    }
    Ok(())
}

struct Sqlite(Connection);

fn row(row: &Row<'_>) -> rusqlite::Result<Punishment> {
    let kind = match row.get_ref(1)?.as_str()? {
        "ban" => Kind::Ban,
        "mute" => Kind::Mute,
        "gag" => Kind::Gag,
        unknown => return Err(rusqlite::Error::InvalidParameterName(unknown.into())),
    };
    let steam_id: i64 = row.get(0)?;
    Ok(Punishment {
        steam_id: steam_id
            .try_into()
            .map_err(|_| rusqlite::Error::IntegralValueOutOfRange(0, steam_id))?,
        kind,
        actor: row
            .get::<_, Option<i64>>(2)?
            .map(|id| {
                u64::try_from(id).map_err(|_| rusqlite::Error::IntegralValueOutOfRange(2, id))
            })
            .transpose()?,
        reason: row.get(3)?,
        created_at: row.get(4)?,
        expires_at: row.get(5)?,
    })
}

const ACTIVE: &str = "SELECT steam_id, kind, actor, reason, created_at, expires_at
       FROM punishments WHERE revoked_at IS NULL AND (expires_at IS NULL OR expires_at > ?1)";

impl PunishmentStore for Sqlite {
    fn active(&self, steam_id: u64, now: i64) -> Result<Vec<Punishment>, StoreError> {
        let mut stmt = self
            .0
            .prepare_cached(&format!("{ACTIVE} AND steam_id = ?2 ORDER BY id"))
            .map_err(err)?;
        stmt.query_map(params![now, i64::try_from(steam_id).map_err(err)?], row)
            .map_err(err)?
            .collect::<Result<_, _>>()
            .map_err(err)
    }

    fn list_bans(
        &self,
        now: i64,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Punishment>, StoreError> {
        let mut stmt = self
            .0
            .prepare_cached(&format!(
                "{ACTIVE} AND kind = 'ban' ORDER BY id DESC LIMIT ?2 OFFSET ?3"
            ))
            .map_err(err)?;
        stmt.query_map(
            params![
                now,
                i64::try_from(limit).map_err(err)?,
                i64::try_from(offset).map_err(err)?
            ],
            row,
        )
        .map_err(err)?
        .collect::<Result<_, _>>()
        .map_err(err)
    }

    fn mutate(&mut self, mutation: Mutation<'_>, now: i64) -> Result<usize, StoreError> {
        let tx = self.0.transaction().map_err(err)?;
        let revoke = |steam_id: u64,
                      kind: Kind,
                      actor: Option<u64>,
                      active_only|
         -> Result<usize, StoreError> {
            tx.execute(
                "UPDATE punishments SET revoked_at = ?1, revoked_by = ?2
                   WHERE steam_id = ?3 AND kind = ?4 AND revoked_at IS NULL
                   AND (NOT ?5 OR expires_at IS NULL OR expires_at > ?1)",
                params![
                    now,
                    actor.map(i64::try_from).transpose().map_err(err)?,
                    i64::try_from(steam_id).map_err(err)?,
                    kind.key(),
                    active_only
                ],
            )
            .map_err(err)
        };
        let mut changed = 0;
        match mutation {
            Mutation::Apply(rows) => {
                for record in rows {
                    revoke(record.steam_id, record.kind, record.actor, false)?;
                    changed += tx.execute(
                        "INSERT INTO punishments (steam_id, kind, actor, reason, created_at, expires_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        params![i64::try_from(record.steam_id).map_err(err)?, record.kind.key(), record.actor.map(i64::try_from).transpose().map_err(err)?, record.reason, record.created_at, record.expires_at]
                    ).map_err(err)?;
                }
            }
            Mutation::Revoke {
                steam_id,
                kinds,
                actor,
            } => {
                for &kind in kinds {
                    changed += revoke(steam_id, kind, actor, true)?;
                }
            }
        }
        tx.commit().map_err(err)?;
        Ok(changed)
    }
}

struct Records {
    path: PathBuf,
    rows: Vec<Punishment>,
}

impl Records {
    fn load(path: PathBuf, ban: bool) -> Result<Self, StoreError> {
        let rows: Vec<Punishment> = match File::open(&path) {
            Ok(file) => serde_json::from_reader(io::BufReader::new(file)).map_err(err)?,
            Err(error) if error.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(error) => return Err(err(error)),
        };
        let mut keys = std::collections::HashSet::new();
        if rows.iter().any(|row| {
            (row.kind == Kind::Ban) != ban || !keys.insert((row.steam_id, row.kind.key()))
        }) {
            return Err(err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid punishment rows: {}", path.display()),
            )));
        }
        Ok(Self { path, rows })
    }

    fn replace(&mut self, rows: Vec<Punishment>) -> Result<(), StoreError> {
        create_parent(&self.path)?;
        let parent = self
            .path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let file = tempfile::NamedTempFile::new_in(parent).map_err(err)?;
        serde_json::to_writer_pretty(&file, &rows).map_err(err)?;
        file.as_file().sync_all().map_err(err)?;
        file.persist(&self.path).map_err(err)?;
        self.rows = rows;
        Ok(())
    }
}

struct Files {
    bans: Records,
    comms: Records,
}

impl PunishmentStore for Files {
    fn active(&self, steam_id: u64, now: i64) -> Result<Vec<Punishment>, StoreError> {
        Ok(self
            .bans
            .rows
            .iter()
            .chain(&self.comms.rows)
            .filter(|p| p.steam_id == steam_id && p.active_at(now))
            .cloned()
            .collect())
    }

    fn list_bans(
        &self,
        now: i64,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Punishment>, StoreError> {
        Ok(self
            .bans
            .rows
            .iter()
            .rev()
            .filter(|p| p.active_at(now))
            .skip(offset)
            .take(limit)
            .cloned()
            .collect())
    }

    fn mutate(&mut self, mutation: Mutation<'_>, now: i64) -> Result<usize, StoreError> {
        let (steam_id, kinds) = match &mutation {
            Mutation::Apply(rows) => match rows.first() {
                Some(first) if rows.iter().all(|row| row.steam_id == first.steam_id) => {
                    (first.steam_id, rows.iter().map(|row| row.kind).collect())
                }
                Some(_) => {
                    return Err(err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "mutation spans multiple players",
                    )));
                }
                None => return Ok(0),
            },
            Mutation::Revoke {
                steam_id, kinds, ..
            } => (*steam_id, kinds.to_vec()),
        };
        let records = if kinds == [Kind::Ban] {
            &mut self.bans
        } else if kinds.contains(&Kind::Ban) {
            return Err(err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "mutation spans ban and communication files",
            )));
        } else {
            &mut self.comms
        };
        let mut next: Vec<_> = records
            .rows
            .iter()
            .filter(|row| row.active_at(now))
            .cloned()
            .collect();
        let before = next.len();
        next.retain(|row| row.steam_id != steam_id || !kinds.contains(&row.kind));
        let changed = match mutation {
            Mutation::Apply(rows) => {
                next.extend_from_slice(rows);
                rows.len()
            }
            Mutation::Revoke { .. } => before - next.len(),
        };
        if changed != 0 {
            records.replace(next)?;
        }
        Ok(changed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contract(open: impl Fn() -> Box<dyn PunishmentStore>) {
        let p = |steam_id, kind, expiry| Punishment {
            steam_id,
            kind,
            expires_at: expiry,
            actor: None,
            reason: "test".into(),
            created_at: 1,
        };
        let a = 76_561_198_000_000_000;
        let b = a + 1;
        let mut store = open();
        store
            .mutate(Mutation::Apply(&[p(a, Kind::Ban, Some(10))]), 1)
            .unwrap();
        store
            .mutate(Mutation::Apply(&[p(b, Kind::Ban, None)]), 2)
            .unwrap();
        store
            .mutate(
                Mutation::Apply(&[p(a, Kind::Mute, Some(9)), p(a, Kind::Gag, None)]),
                3,
            )
            .unwrap();
        drop(store);
        let mut store = open();
        assert_eq!(store.active(a, 8).unwrap().len(), 3);
        assert_eq!(store.active(a, 9).unwrap().len(), 2);
        assert_eq!(
            store
                .active(a, 10)
                .unwrap()
                .iter()
                .map(|p| p.kind)
                .collect::<Vec<_>>(),
            [Kind::Gag]
        );
        assert_eq!(store.list_bans(9, 1, 1).unwrap()[0].steam_id, a);
        assert!(store.list_bans(10, 1, 1).unwrap().is_empty());
        assert_eq!(
            store
                .mutate(
                    Mutation::Revoke {
                        steam_id: a,
                        kinds: &[Kind::Mute, Kind::Gag],
                        actor: Some(b)
                    },
                    9
                )
                .unwrap(),
            1
        );
        assert!(open().active(a, 10).unwrap().is_empty());
    }

    #[test]
    fn sqlite_persists_and_retains_revocation_history() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("admin.db");
        contract(|| sqlite(&db).unwrap());
        let conn = Connection::open(db).unwrap();
        let mut stmt = conn
            .prepare("SELECT kind, expires_at, revoked_at, revoked_by FROM punishments ORDER BY id")
            .unwrap();
        let history: Vec<_> = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<i64>>(1)?,
                    row.get::<_, Option<i64>>(2)?,
                    row.get::<_, Option<i64>>(3)?,
                ))
            })
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert_eq!(
            history,
            [
                ("ban".into(), Some(10), None, None),
                ("ban".into(), None, None, None),
                ("mute".into(), Some(9), None, None),
                ("gag".into(), None, Some(9), Some(76_561_198_000_000_001)),
            ]
        );
    }

    #[test]
    fn files_persist_both_restrictions_and_do_not_accept_corrupt_input() {
        let dir = tempfile::tempdir().unwrap();
        contract(|| files(dir.path()).unwrap());
        let path = dir.path().join("bans.json");
        fs::write(&path, "[{").unwrap();
        assert!(files(dir.path()).is_err());
        assert_eq!(fs::read_to_string(path).unwrap(), "[{");
    }

    #[test]
    fn failed_atomic_replace_does_not_publish_in_memory() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("comms.json");
        let mut store = files(dir.path()).unwrap();
        fs::create_dir(&path).unwrap();
        let row = Punishment {
            steam_id: 1,
            kind: Kind::Mute,
            actor: None,
            expires_at: None,
            created_at: 1,
            reason: "test".into(),
        };
        let gag = Punishment {
            kind: Kind::Gag,
            ..row.clone()
        };
        assert!(store.mutate(Mutation::Apply(&[row, gag]), 1).is_err());
        assert!(store.active(1, 1).unwrap().is_empty());
        fs::remove_dir(&path).unwrap();
        assert!(files(dir.path()).unwrap().active(1, 1).unwrap().is_empty());
    }

    #[test]
    fn sqlite_rolls_back_replacement_batch_if_second_write_fails() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("admin.db");
        let mut store = sqlite(&path).unwrap();
        let original = Punishment {
            steam_id: 76_561_198_000_000_000,
            kind: Kind::Mute,
            actor: None,
            reason: "original".into(),
            created_at: 1,
            expires_at: None,
        };
        store
            .mutate(Mutation::Apply(std::slice::from_ref(&original)), 1)
            .unwrap();
        Connection::open(&path)
            .unwrap()
            .execute_batch(
                "CREATE TRIGGER reject_gag BEFORE INSERT ON punishments
             WHEN NEW.kind = 'gag' BEGIN SELECT RAISE(ABORT, 'test injected write failure'); END;",
            )
            .unwrap();
        let replacement = Punishment {
            reason: "replacement".into(),
            created_at: 2,
            ..original.clone()
        };
        let gag = Punishment {
            kind: Kind::Gag,
            ..replacement.clone()
        };
        let error = store
            .mutate(Mutation::Apply(&[replacement, gag]), 2)
            .unwrap_err();
        assert!(error.to_string().contains("test injected write failure"));
        drop(store);
        assert_eq!(
            sqlite(&path).unwrap().active(original.steam_id, 2).unwrap(),
            [original]
        );
    }

    #[test]
    fn both_drivers_filter_before_paging_after_reopen() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("admin.db");
        let open_sqlite = || sqlite(&db).unwrap();
        let open_files = || files(dir.path()).unwrap();
        for open in [
            &open_sqlite as &dyn Fn() -> Box<dyn PunishmentStore>,
            &open_files,
        ] {
            let mut store = open();
            let base = 76_561_198_000_000_000;
            for index in 0..9 {
                let ban = Punishment {
                    steam_id: base + index,
                    kind: Kind::Ban,
                    actor: None,
                    reason: format!("entry {index}"),
                    created_at: index as i64,
                    expires_at: (index == 4).then_some(10),
                };
                store.mutate(Mutation::Apply(&[ban]), index as i64).unwrap();
            }
            drop(store);
            let store = open();
            let ids = |time, offset| {
                store
                    .list_bans(time, offset, 8)
                    .unwrap()
                    .into_iter()
                    .map(|ban| ban.steam_id - base)
                    .collect::<Vec<_>>()
            };
            assert_eq!(ids(9, 0), [8, 7, 6, 5, 4, 3, 2, 1]);
            assert_eq!(ids(9, 8), [0]);
            assert_eq!(ids(10, 0), [8, 7, 6, 5, 3, 2, 1, 0]);
            assert!(ids(10, 8).is_empty());
        }
    }
}
