use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub slot: i32,
    pub user_id: i32,
    pub steam_id: Option<u64>,
    pub name: String,
    pub team: Team,
    pub alive: bool,
    pub bot: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Unassigned,
    Spectator,
    Terrorist,
    CounterTerrorist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    Console,
    Player(i32),
}

#[derive(Debug, Clone, Copy)]
pub struct Policy {
    pub allow_many: bool,
    pub allow_self: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TargetError {
    NotFound,
    UnknownSelector,
    GroupForbidden,
    Ambiguous(Vec<String>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Selection<'a> {
    pub allowed: Vec<&'a Player>,
    pub blocked: Vec<&'a Player>,
}

pub fn steam_id(text: &str) -> Option<u64> {
    // Public-universe individual Steam account, desktop instance.
    const BASE: u64 = 76_561_197_960_265_728;
    let id = (text.len() == 17 && text.bytes().all(|c| c.is_ascii_digit()))
        .then(|| text.parse::<u64>().ok())
        .flatten()?;
    (BASE + 1..=BASE + u64::from(u32::MAX))
        .contains(&id)
        .then_some(id)
}

pub fn select<'a>(
    query: &str,
    actor: Actor,
    players: &'a [Player],
    policy: Policy,
    mut can_affect: impl FnMut(&Player) -> bool,
) -> Result<Selection<'a>, TargetError> {
    let query = query.trim().to_lowercase();
    let is_self = |p: &Player| actor == Actor::Player(p.slot);
    let group = query.starts_with('@') || query.starts_with("!@");
    let matches: Vec<_> = if group {
        let predicate: fn(&Player) -> bool = match query.as_str() {
            "@all" | "@me" | "!@me" => |_| true,
            "@t" => |p| p.team == Team::Terrorist,
            "@ct" => |p| p.team == Team::CounterTerrorist,
            "@spec" => |p| p.team == Team::Spectator,
            "@alive" => |p| p.alive,
            "@dead" => |p| !p.alive && matches!(p.team, Team::Terrorist | Team::CounterTerrorist),
            "@bots" => |p| p.bot,
            "@humans" => |p| !p.bot,
            _ => return Err(TargetError::UnknownSelector),
        };
        if !policy.allow_many && query != "@me" {
            return Err(TargetError::GroupForbidden);
        }
        players
            .iter()
            .filter(|p| match query.as_str() {
                "@me" => is_self(p),
                "!@me" => !is_self(p),
                _ => predicate(p),
            })
            .collect()
    } else if let Some(value) = query.strip_prefix('#') {
        let (number, slots_only, ids_only) = match value.split_once(':') {
            Some(("slot", number)) => (number, true, false),
            Some(("userid", number)) => (number, false, true),
            _ => (value, false, false),
        };
        let number = number
            .parse::<i32>()
            .ok()
            .filter(|n| *n >= 0)
            .ok_or(TargetError::NotFound)?;
        // A bare #N may name a userid and another player's slot. Never pick one silently.
        players
            .iter()
            .filter(|p| (!ids_only && p.slot == number) || (!slots_only && p.user_id == number))
            .collect()
    } else if query.len() == 17 && query.bytes().all(|c| c.is_ascii_digit()) {
        let id = steam_id(&query).ok_or(TargetError::NotFound)?;
        players.iter().filter(|p| p.steam_id == Some(id)).collect()
    } else if !query.is_empty() {
        players
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&query))
            .collect()
    } else {
        Vec::new()
    };
    if matches.is_empty() {
        return Err(TargetError::NotFound);
    }
    if !group && matches.len() > 1 {
        return Err(TargetError::Ambiguous(
            matches
                .iter()
                .map(|p| format!("{} (#userid:{})", p.name, p.user_id))
                .collect(),
        ));
    }
    let mut seen = HashSet::new();
    let (allowed, blocked) = matches
        .into_iter()
        .filter(|p| seen.insert(p.slot))
        .partition(|p| {
            if is_self(p) && !policy.allow_self {
                return false;
            }
            actor == Actor::Console || can_affect(p)
        });
    Ok(Selection { allowed, blocked })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn player(slot: i32, user_id: i32, name: &str, bot: bool) -> Player {
        Player {
            slot,
            user_id,
            steam_id: None,
            name: name.into(),
            team: Team::CounterTerrorist,
            alive: true,
            bot,
        }
    }

    #[test]
    fn rejects_slot_userid_collision_and_fuzzy_ambiguity_before_immunity() {
        let players = [
            player(1, 2, "Alpha", false),
            player(2, 3, "ALPHAbet", false),
        ];
        let policy = Policy {
            allow_many: false,
            allow_self: true,
        };
        for input in ["#2", "ALPHA"] {
            assert!(
                matches!(select(input, Actor::Player(0), &players, policy, |p| p.slot == 2), Err(TargetError::Ambiguous(names)) if names.len() == 2)
            );
        }
        assert_eq!(
            select("#userid:2", Actor::Console, &players, policy, |_| false)
                .unwrap()
                .allowed[0]
                .slot,
            1
        );
        assert_eq!(
            select("#slot:2", Actor::Console, &players, policy, |_| false)
                .unwrap()
                .allowed[0]
                .slot,
            2
        );
    }

    #[test]
    fn filters_self_and_immune_players_but_retains_eligible_players() {
        let players = [
            player(0, 4, "OP", false),
            player(1, 5, "Immune", false),
            player(2, 6, "Target", true),
        ];
        let mut policy = Policy {
            allow_many: true,
            allow_self: false,
        };
        let result = select("@ct", Actor::Player(0), &players, policy, |p| p.slot != 1).unwrap();
        assert_eq!(result.allowed, [&players[2]]);
        assert_eq!(result.blocked, [&players[0], &players[1]]);
        policy.allow_many = false;
        assert_eq!(
            select("@bots", Actor::Player(0), &players, policy, |_| true),
            Err(TargetError::GroupForbidden)
        );
        assert!(
            select("@me", Actor::Player(0), &players, policy, |_| true)
                .unwrap()
                .allowed
                .is_empty()
        );
    }

    #[test]
    fn parses_only_public_individual_steam_ids() {
        assert_eq!(steam_id("76561198000000000"), Some(76_561_198_000_000_000));
        for invalid in [
            "76561197960265728",
            "99999999999999999",
            "7656119800000000x",
        ] {
            assert_eq!(steam_id(invalid), None);
        }
    }

    #[test]
    fn steam_id_and_quoted_nickname_resolve_the_same_authenticated_player() {
        let players = [
            Player {
                steam_id: Some(76_561_198_000_000_000),
                ..player(0, 15, "Player With Spaces", false)
            },
            player(1, 16, "Player", true),
        ];
        let policy = Policy {
            allow_many: false,
            allow_self: false,
        };
        let words = crate::command::words(r#"kick "PLAYER WITH SPACES" test"#).unwrap();
        for input in ["76561198000000000", &words[1]] {
            let target = select(input, Actor::Console, &players, policy, |_| false).unwrap();
            assert_eq!(target.allowed, [&players[0]]);
        }
        assert_eq!(
            select(
                "76561198000000001",
                Actor::Console,
                &players,
                policy,
                |_| true
            ),
            Err(TargetError::NotFound)
        );
    }

    #[test]
    fn group_selectors_respect_team_liveness_bot_status_and_self() {
        let players = [
            Player {
                team: Team::Terrorist,
                ..player(0, 10, "OP", false)
            },
            Player {
                alive: false,
                ..player(1, 11, "Dead", true)
            },
            Player {
                alive: false,
                team: Team::Spectator,
                ..player(2, 12, "Spectator", false)
            },
        ];
        for (query, expected) in [
            ("@all", &[0, 1, 2][..]),
            ("@me", &[0][..]),
            ("!@me", &[1, 2][..]),
            ("@t", &[0][..]),
            ("@ct", &[1][..]),
            ("@spec", &[2][..]),
            ("@alive", &[0][..]),
            ("@dead", &[1][..]),
            ("@bots", &[1][..]),
            ("@humans", &[0, 2][..]),
        ] {
            let policy = Policy {
                allow_many: true,
                allow_self: true,
            };
            let found = select(query, Actor::Player(0), &players, policy, |_| true).unwrap();
            assert_eq!(
                found.allowed.iter().map(|p| p.slot).collect::<Vec<_>>(),
                expected,
                "{query}"
            );
        }
    }
}
