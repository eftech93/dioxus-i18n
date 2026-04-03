use std::collections::HashMap;
use std::path::PathBuf;

use crate::loader::Translations;

/// Configuration for the i18n provider.
#[derive(Clone, Debug, PartialEq)]
pub struct I18nConfig {
    /// Directory containing JSON locale files (e.g. `./locales`).
    /// On WASM this is ignored if [`initial`](Self::initial) is set.
    pub locales_dir: PathBuf,
    /// Default locale code to use before the user selects one.
    pub default_locale: String,
    /// Optional pre-loaded translations.
    /// Useful for embedding JSON files at compile time with `include_str!`.
    pub initial: Option<HashMap<String, Translations>>,
}

impl I18nConfig {
    /// Create a new config that loads translations from disk.
    pub fn new(locales_dir: impl Into<PathBuf>, default_locale: impl Into<String>) -> Self {
        Self {
            locales_dir: locales_dir.into(),
            default_locale: default_locale.into(),
            initial: None,
        }
    }

    /// Create a new config with pre-loaded translations.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut translations = HashMap::new();
    /// translations.insert("en".to_string(), dioxus_i18n_json::parse_translation_json(include_str!("locales/en.json")).unwrap());
    /// let config = I18nConfig::embedded("en", translations);
    /// ```
    pub fn embedded(
        default_locale: impl Into<String>,
        translations: HashMap<String, Translations>,
    ) -> Self {
        Self {
            locales_dir: PathBuf::new(),
            default_locale: default_locale.into(),
            initial: Some(translations),
        }
    }
}
