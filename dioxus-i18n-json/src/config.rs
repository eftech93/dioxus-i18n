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
    /// Optional locale to fall back to when a key is missing in the active locale.
    pub fallback_locale: Option<String>,
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
            fallback_locale: None,
            initial: None,
        }
    }

    /// Set a fallback locale for missing keys.
    ///
    /// # Example
    /// ```rust,ignore
    /// let config = I18nConfig::new("./locales", "es")
    ///     .with_fallback_locale("en");
    /// ```
    pub fn with_fallback_locale(mut self, locale: impl Into<String>) -> Self {
        self.fallback_locale = Some(locale.into());
        self
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
            fallback_locale: None,
            initial: Some(translations),
        }
    }
}
