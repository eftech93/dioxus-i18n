# dioxus-i18n-json

> A lightweight, reactive internationalization (i18n) library for [Dioxus](https://dioxuslabs.com/) with JSON-based translations, hot-reload, and compile-time typed keys.

## Features

- 🌍 **JSON locale files** — Simple nested objects, accessed via dot-notation keys.
- 🪝 **Hook-style API** — `use_i18n()` and `use_t()` feel just like React i18n libraries.
- 🔢 **Plurals** — Powered by Unicode CLDR plural rules (`one`, `other`, `few`, `many`, `two`, `zero`).
- 📝 **String interpolation** — Inject variables with `{name}` placeholders.
- 🧩 **Component wrappers** — `<Trans>` lets you embed Dioxus components inside translations.
- ⌨️ **Typed keys** — `generate_keys!` macro gives you IDE autocomplete and compile-time validation.
- 🛡️ **Fallback locale** — Missing keys automatically fall back to a secondary locale before returning the raw key.
- ♻️ **Hot reload** — Edit locale files while the desktop app runs and see changes instantly.
- ⚡ **Reactive** — Locale changes trigger Dioxus signals, so the UI re-renders automatically.

## What's New in 0.0.5

- **Robust `generate_keys!` macro** — The proc-macro now safely handles edge-case JSON keys, including special characters (`-`, `.`, spaces), duplicate identifiers, and empty/underscore-only keys.
- **Fixed Docsify site** — Documentation now renders correctly on GitHub Pages with a styled coverpage.

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
dioxus-i18n-json = "0.0.5"
```

For desktop hot-reload during development:

```toml
[dependencies]
dioxus-i18n-json = { version = "0.0.5", features = ["hot-reload"] }
```

## Workspace Crates

This project is split into two crates so the proc-macro can be compiled separately:

- **`dioxus-i18n-json`** — The main library with hooks, provider, and components.
- **`dioxus-i18n-json-macro`** — The proc-macro crate that provides `generate_keys!`.

See [Workspace Crates](crates.md) for details on when to depend on each one.

## Next Steps

- Read the [Getting Started](getting-started.md) guide.
- Browse the [API Reference](api.md).
- See [Examples](examples.md) for desktop, web, and advanced use-cases.
