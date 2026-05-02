use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

/// A translation value for a single key: either a plain string or a plural form.
#[derive(Clone, Debug, PartialEq)]
pub enum TranslationValue {
    String(String),
    Plural(PluralForms),
}

/// Holds the different plural variants for a key.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PluralForms {
    pub zero: Option<String>,
    pub one: Option<String>,
    pub two: Option<String>,
    pub few: Option<String>,
    pub many: Option<String>,
    pub other: Option<String>,
}

impl PluralForms {
    /// Get the form for a given plural category, falling back to `other`.
    pub fn get(&self, category: &str) -> Option<&str> {
        let form = match category {
            "zero" => self.zero.as_deref(),
            "one" => self.one.as_deref(),
            "two" => self.two.as_deref(),
            "few" => self.few.as_deref(),
            "many" => self.many.as_deref(),
            "other" => self.other.as_deref(),
            _ => None,
        };
        form.or(self.other.as_deref())
    }
}

/// Flattened key-value translations for a single locale.
pub type Translations = HashMap<String, TranslationValue>;

/// Load every `*.json` file in `dir` as a locale.
///
/// The file stem becomes the locale code (e.g. `en.json` -> `en`).
/// Nested objects are flattened into dot-notation keys, except objects
/// that represent plural forms (keys are `zero`, `one`, `two`, `few`, `many`, `other`).
pub fn load_translations<P: AsRef<Path>>(
    dir: P,
) -> Result<HashMap<String, Translations>, LoadError> {
    let mut all = HashMap::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let locale = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            let content = std::fs::read_to_string(&path)?;
            let value: Value = serde_json::from_str(&content)?;
            let mut flat = Translations::new();
            flatten_json_value(&value, "", &mut flat);
            all.insert(locale, flat);
        }
    }
    Ok(all)
}

pub fn parse_translation_json(json: &str) -> Result<Translations, LoadError> {
    let value: Value = serde_json::from_str(json)?;
    let mut flat = Translations::new();
    flatten_json_value(&value, "", &mut flat);
    Ok(flat)
}

fn is_plural_object(obj: &serde_json::Map<String, Value>) -> bool {
    if obj.is_empty() {
        return false;
    }
    obj.keys().all(|k| {
        matches!(
            k.as_str(),
            "zero" | "one" | "two" | "few" | "many" | "other"
        )
    })
}

pub(crate) fn flatten_json_value(value: &Value, prefix: &str, map: &mut Translations) {
    match value {
        Value::Object(obj) => {
            if is_plural_object(obj) {
                let mut pf = PluralForms::default();
                for (k, v) in obj {
                    if let Value::String(s) = v {
                        match k.as_str() {
                            "zero" => pf.zero = Some(s.clone()),
                            "one" => pf.one = Some(s.clone()),
                            "two" => pf.two = Some(s.clone()),
                            "few" => pf.few = Some(s.clone()),
                            "many" => pf.many = Some(s.clone()),
                            "other" => pf.other = Some(s.clone()),
                            _ => {}
                        }
                    }
                }
                map.insert(prefix.to_string(), TranslationValue::Plural(pf));
            } else {
                for (k, v) in obj {
                    let new_prefix = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    flatten_json_value(v, &new_prefix, map);
                }
            }
        }
        Value::String(s) => {
            map.insert(prefix.to_string(), TranslationValue::String(s.clone()));
        }
        Value::Number(n) => {
            map.insert(prefix.to_string(), TranslationValue::String(n.to_string()));
        }
        Value::Bool(b) => {
            map.insert(prefix.to_string(), TranslationValue::String(b.to_string()));
        }
        Value::Null => {
            map.insert(prefix.to_string(), TranslationValue::String(String::new()));
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let new_prefix = format!("{}[{}]", prefix, i);
                flatten_json_value(v, &new_prefix, map);
            }
        }
    }
}

/// Errors that can occur while loading locale files.
#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "IO error: {}", e),
            LoadError::Json(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl std::error::Error for LoadError {}

impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        LoadError::Io(e)
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(e: serde_json::Error) -> Self {
        LoadError::Json(e)
    }
}
