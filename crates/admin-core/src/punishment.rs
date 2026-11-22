use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Ban,
    Mute,
    Gag,
}

impl Kind {
    pub const fn key(self) -> &'static str {
        match self {
            Self::Ban => "ban",
            Self::Mute => "mute",
            Self::Gag => "gag",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Duration(u64);

impl Duration {
    pub fn parse(input: &str) -> Option<Self> {
        if input == "0" {
            return Some(Self(0));
        }
        let (amount, unit) = input.split_at_checked(input.len().checked_sub(1)?)?;
        let factor = match unit {
            "m" => 60,
            "h" => 3600,
            "d" => 86400,
            "w" => 604800,
            _ => return None,
        };
        let value = amount.parse::<u64>().ok()?.checked_mul(factor)?;
        (value > 0 && i64::try_from(value).is_ok()).then_some(Self(value))
    }

    pub fn from_minutes(minutes: i32) -> Option<Self> {
        u64::try_from(minutes).ok().map(|amount| Self(amount * 60))
    }

    pub fn expiration(self, now: i64) -> Option<Option<i64>> {
        if self.0 == 0 {
            Some(None)
        } else {
            Some(Some(now.checked_add(i64::try_from(self.0).ok()?)?))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Punishment {
    pub steam_id: u64,
    pub kind: Kind,
    pub actor: Option<u64>,
    pub reason: String,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}

impl Punishment {
    pub fn active_at(&self, now: i64) -> bool {
        self.expires_at.is_none_or(|expires| now < expires)
    }
}

#[derive(Debug)]
pub struct StoreError(pub Box<dyn std::error::Error + Send + Sync>);

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for StoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.0)
    }
}

/// All rows in a mutation address the same player. Silence changes both communication
/// restrictions together; callers never announce success after only one was persisted.
pub enum Mutation<'a> {
    Apply(&'a [Punishment]),
    Revoke {
        steam_id: u64,
        kinds: &'a [Kind],
        actor: Option<u64>,
    },
}

pub trait PunishmentStore {
    fn active(&self, steam_id: u64, now: i64) -> Result<Vec<Punishment>, StoreError>;
    fn list_bans(
        &self,
        now: i64,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Punishment>, StoreError>;
    fn mutate(&mut self, mutation: Mutation<'_>, now: i64) -> Result<usize, StoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_seconds_and_expires_at_the_boundary() {
        for (token, seconds) in [("10m", 600), ("2h", 7200), ("7d", 604800), ("1w", 604800)] {
            assert_eq!(
                Duration::parse(token).unwrap().expiration(100),
                Some(Some(100 + seconds))
            );
        }
        assert_eq!(
            Duration::parse("0").unwrap().expiration(i64::MAX),
            Some(None)
        );
        assert_eq!(Duration::parse("1m").unwrap().expiration(i64::MAX), None);
        let rule = Punishment {
            steam_id: 1,
            kind: Kind::Gag,
            actor: None,
            reason: String::new(),
            created_at: 10,
            expires_at: Some(20),
        };
        assert!(rule.active_at(19));
        assert!(!rule.active_at(20));
    }

    #[test]
    fn rejects_invalid_negative_and_unrepresentable_durations() {
        for token in [
            "",
            "💥",
            "-1h",
            "0d",
            "10",
            "3s",
            "1m\n",
            "18446744073709551616m", // Cannot parse the amount.
            "18446744073709551615w", // Cannot multiply by the unit.
            "15250284452472w",       // Seconds cannot fit in a Unix timestamp.
        ] {
            assert_eq!(Duration::parse(token), None, "{token:?}");
        }
        assert_eq!(Duration::from_minutes(-1), None);
        assert_eq!(
            Duration::from_minutes(0).unwrap().expiration(100),
            Some(None)
        );
        assert_eq!(
            Duration::from_minutes(2).unwrap().expiration(100),
            Some(Some(220))
        );
    }
}
