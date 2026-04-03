# dioxus-i18n-json

> A lightweight, reactive internationalization (i18n) library for [Dioxus](https://dioxuslabs.com/) with JSON-based translations, hot-reload, and compile-time typed keys.

## Features

- 🌍 **JSON locale files** — Simple nested objects, accessed via dot-notation keys.
- 🪝 **Hook-style API** — `use_i18n()` and `use_t()` feel just like React i18n libraries.
- 🔢 **Plurals** — Powered by Unicode CLDR plural rules (`one`, `other`, `few`, `many`, `two`, `zero`).
- 📝 **String interpolation** — Inject variables with `{name}` placeholders.
- 🧩 **Component wrappers** — `<Trans>` lets you embed Dioxus components inside translations.
- ⌨️ **Typed keys** — `generate_keys!` macro gives you IDE autocomplete and compile-time validation.
- ♻️ **Hot reload** — Edit locale files while the desktop app runs and see changes instantly.
- ⚡ **Reactive** — Locale changes trigger Dioxus signals, so the UI re-renders automatically.

## Quick Example

```rust
use dioxus::prelude::*;
use dioxus_i18n_json::{generate_keys, I18nConfig, I18nProvider, use_t, UseI18n};

generate_keys!("locales/en.json");

fn App() -> Element {
    rsx! {
        I18nProvider {
            config: I18nConfig::new("./locales", "en"),
            Hello {}
        }
    }
}

#[component]
fn Hello() -> Element {
    let UseI18n { t, .. } = use_t();
    rsx! {
        h1 { {t(keys::greeting)} }
    }
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-i18n-json = "0.0.2"
```

For desktop hot-reload during development:

```toml
[dependencies]
dioxus-i18n-json = { version = "0.0.2", features = ["hot-reload"] }
```

## Next Steps

- Read the [Getting Started](getting-started.md) guide.
- Browse the [API Reference](api.md).
- See [Examples](examples.md) for desktop, web, and advanced use-cases.
