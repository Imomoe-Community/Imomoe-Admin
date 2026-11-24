use imomoe_admin_core::punishment::Duration;
use serde::{Deserialize, Deserializer, de::Error as _};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "driver", rename_all = "snake_case", deny_unknown_fields)]
pub enum Storage {
    Sqlite { path: PathBuf },
    File {},
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Admin {
    #[serde(default, rename = "$schema")]
    _schema: String,
    pub storage: Storage,
    pub default_ban: String,
    pub default_comms: String,
    #[serde(deserialize_with = "unique_strings")]
    pub allow_self: HashSet<String>,
    pub hide_on_join: bool,
    #[serde(deserialize_with = "unique_strings")]
    pub cvar_allowlist: HashSet<String>,
    pub report_cooldown_seconds: u64,
    pub audit_log: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Announcements {
    #[serde(default, rename = "$schema")]
    _schema: String,
    pub interval_seconds: u64,
    pub messages: Vec<String>,
    pub rules: Vec<String>,
    pub help: Vec<String>,
}

pub struct Config {
    pub admin: Admin,
    pub announcements: Announcements,
    translations: [HashMap<String, String>; 2],
    pub default_ban: Duration,
    pub default_comms: Duration,
}

fn unique_strings<'de, D: Deserializer<'de>>(
    de: D,
) -> std::result::Result<HashSet<String>, D::Error> {
    let mut unique = HashSet::new();
    for item in Vec::<String>::deserialize(de)? {
        if !unique.insert(item) {
            return Err(D::Error::custom("duplicate array item"));
        }
    }
    Ok(unique)
}

fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let source = fs::read_to_string(path)
        .map_err(|error| io::Error::new(error.kind(), format!("{}: {error}", path.display())))?;
    let options = jsonc_parser::ParseOptions {
        allow_comments: true,
        allow_trailing_commas: true,
        allow_loose_object_property_names: false,
        allow_missing_commas: false,
        allow_single_quoted_strings: false,
        allow_hexadecimal_numbers: false,
        allow_unary_plus_numbers: false,
        allow_bare_decimal_point_numbers: false,
        allow_non_finite_numbers: false,
        allow_extended_string_escapes: false,
    };
    jsonc_parser::parse_to_serde_value(&source, &options)
        .map_err(|error| format!("{}: {error}", path.display()).into())
}

#[derive(Deserialize)]
struct TranslationSchema {
    required: Vec<String>,
}

fn read_translations(path: &Path, required: &[String]) -> Result<HashMap<String, String>> {
    let mut phrases: HashMap<String, String> = read(path)?;
    phrases.remove("$schema");
    for key in required {
        if phrases.get(key).is_none_or(String::is_empty) {
            return Err(format!("{}: missing or empty translation {key}", path.display()).into());
        }
    }
    if let Some(key) = phrases.keys().find(|key| !required.contains(key)) {
        return Err(format!("{}: unknown translation {key}", path.display()).into());
    }
    Ok(phrases)
}

fn default_duration(value: &str) -> Option<Duration> {
    if value == "0" || matches!(value.as_bytes().first(), Some(b'1'..=b'9')) {
        Duration::parse(value)
    } else {
        None
    }
}

impl Config {
    pub fn load(root: &Path) -> Result<Self> {
        let admin: Admin = read(&root.join("admin.jsonc"))?;
        let announcements: Announcements = read(&root.join("announcements.jsonc"))?;
        let schema: TranslationSchema = serde_json::from_str(include_str!(
            "../../../configs/imomoe_admin/schemas/translations.schema.json"
        ))?;
        let translations = [
            read_translations(&root.join("translations/en.jsonc"), &schema.required)?,
            read_translations(&root.join("translations/zh-CN.jsonc"), &schema.required)?,
        ];
        let default_ban =
            default_duration(&admin.default_ban).ok_or("invalid default_ban duration")?;
        let default_comms =
            default_duration(&admin.default_comms).ok_or("invalid default_comms duration")?;
        if announcements.interval_seconds == 0 && !announcements.messages.is_empty() {
            return Err(
                "announcement interval must be positive when announcements are enabled".into(),
            );
        }
        if let Storage::Sqlite { path } = &admin.storage
            && (path.as_os_str().is_empty()
                || path.is_absolute()
                || path
                    .components()
                    .any(|part| !matches!(part, std::path::Component::Normal(_))))
        {
            return Err(
                "storage.path must be a relative file path inside the server directory".into(),
            );
        }
        if admin.cvar_allowlist.iter().any(|name| {
            name.is_empty()
                || !name
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        }) {
            return Err("cvar_allowlist accepts lowercase ConVar names only".into());
        }
        Ok(Self {
            admin,
            announcements,
            translations,
            default_ban,
            default_comms,
        })
    }

    pub fn phrase<'a>(&'a self, language: &str, key: &'a str) -> &'a str {
        self.translations[usize::from(is_chinese(language))]
            .get(key)
            .or_else(|| self.translations[0].get(key))
            .map(String::as_str)
            .unwrap_or(key)
    }
}

pub(crate) fn is_chinese(language: &str) -> bool {
    language
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("zh"))
}
