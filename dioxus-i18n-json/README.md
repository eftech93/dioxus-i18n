# dioxus-i18n-json

A lightweight internationalization (i18n) library for [Dioxus](https://dioxuslabs.com/) with JSON-based translations, hot-reload support, typed keys, and component wrappers.

📚 **Full documentation:** [https://eftech93.github.io/dioxus-i18n](https://eftech93.github.io/dioxus-i18n)

## Features

- 🌍 **JSON locale files** – Simple nested objects, accessed via dot-notation keys.
- 🪝 **Hook-style API** – `use_i18n()`, `use_t()`, `use_t_ns()` feel just like React i18n libraries.
- 🔢 **Plurals** – Uses Unicode CLDR plural rules (`one`, `other`, `few`, `many`, `two`, `zero`).
- 📝 **String interpolation** – Inject variables with `{name}` placeholders.
- 🧩 **Component wrappers** – `<Trans>` lets you embed Dioxus components inside translations.
- ⌨️ **Typed keys / autocomplete** – `generate_keys!` macro turns your JSON into a typed `keys` module.
- 🛡️ **Fallback locale** – Missing keys automatically fall back to a secondary locale before returning the raw key.
- ♻️ **Hot reload** – Edit locale files while the desktop app is running (enabled via the `hot-reload` feature).
- ⚡ **Reactive** – Locale changes trigger Dioxus signal updates automatically.

## Quick start

### 1. Add the dependency

```toml
[dependencies]
dioxus-i18n-json = "0.0.5"
```

For desktop hot-reload:

```toml
[dependencies]
dioxus-i18n-json = { version = "0.0.5", features = ["hot-reload"] }
```

### 2. Create a locale file

`locales/en.json`
```json
{
  "greeting": "Hello, {name}!",
  "messages": {
    "welcome": "Welcome to dioxus-i18n-json."
  },
  "itemCount": {
    "one": "You have {count} item.",
    "other": "You have {count} items."
  }
}
```

### 3. Use in your app

```rust
use dioxus::prelude::*;
use dioxus_i18n_json::{generate_keys, I18nConfig, I18nProvider, use_i18n, use_t, UseI18n};

generate_keys!("locales/en.json");

fn App() -> Element {
    rsx! {
        I18nProvider {
            config: I18nConfig::new("./locales", "en"),
            AppContent {}
        }
    }
}

#[component]
fn AppContent() -> Element {
    let mut i18n = use_i18n();
    let UseI18n { t, tp, tf } = use_t();

    rsx! {
        h1 { {tf(keys::greeting, &[("name", "Alice")])} }
        p  { {t(keys::messages::welcome)} }
        p  { {tp(keys::itemCount, 5i64)} }
        button { onclick: move |_| i18n.set_locale("es"), "Switch locale" }
    }
}
```

## Web / WASM support

Browsers cannot read the local filesystem at runtime. Use `I18nConfig::embedded()` with `include_str!`:

```rust
use std::collections::HashMap;
use dioxus_i18n_json::{I18nConfig, I18nProvider, parse_translation_json};

fn App() -> Element {
    let mut translations = HashMap::new();
    translations.insert(
        "en".to_string(),
        parse_translation_json(include_str!("../locales/en.json")).unwrap(),
    );

    rsx! {
        I18nProvider {
            config: I18nConfig::embedded("en", translations),
            Page {}
        }
    }
}
```

## License

MIT OR Apache-2.0

Repository: [https://github.com/eftech93/dioxus-i18n](https://github.com/eftech93/dioxus-i18n)
