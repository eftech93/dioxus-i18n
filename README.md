# dioxus-i18n-json

A lightweight internationalization (i18n) library for [Dioxus](https://dioxuslabs.com/)
with JSON-based translations, hot-reload, typed keys, and component wrappers.

Created by [Esteban Puello](mailto:eftech93@gmail.com).

📚 **Documentation:** [https://eftech93.github.io/dioxus-i18n](https://eftech93.github.io/dioxus-i18n)

## Why `dioxus-i18n-json`?

The simpler `dioxus-i18n` name was already taken on crates.io. This crate emphasizes its JSON-first approach to translations, along with typed keys, hot-reload, and component wrappers.

## Features

- 🌍 **JSON locale files** – Simple nested objects, accessed via dot-notation keys.
- 🪝 **Hook-style API** – `use_i18n()` feels just like React i18n libraries.
- 🔢 **Plurals** – Uses Unicode CLDR plural rules (`one`, `other`, `few`, `many`, `two`, `zero`).
- 📝 **String interpolation** – Inject variables with `{name}` placeholders.
- 🧩 **Component wrappers** – `<Trans>` lets you embed Dioxus components inside translations (e.g. links, bold text).
- ⌨️ **Typed keys / autocomplete** – `generate_keys!` macro turns your JSON into a `keys` module, giving you IDE autocomplete and compile-time validation.
- ♻️ **Hot reload** – Edit locale files while the desktop app is running and see changes instantly (enabled via the `hot-reload` feature).
- ⚡ **Reactive** – Locale changes trigger Dioxus signal updates, so the UI re-renders automatically.

## Workspace layout

```
dioxus-i18n-json/
├── dioxus-i18n-json/     # Core library
│   └── src/
│       ├── lib.rs        # I18nProvider, use_i18n, I18n handle
│       ├── config.rs     # I18nConfig
│       ├── loader.rs     # JSON loading & flattening
│       └── hot_reload.rs # File watcher (behind feature flag)
├── dioxus-i18n-json-macro/  # Proc-macro for generate_keys!
│   └── src/lib.rs
└── examples/
    ├── hello-i18n/       # Desktop example with en/es/fr locales
    ├── minimal-i18n/     # Bare-bones desktop example
    ├── web-i18n/         # Browser example (English / Japanese)
    └── advanced-i18n/    # Complex demo with plurals, Trans, and hot-reload
```

## What's New in 0.0.4

- **Robust `generate_keys!` macro** — The proc-macro now safely handles edge-case JSON keys, including special characters (`-`, `.`, spaces), duplicate identifiers, and empty/underscore-only keys.
- **Fixed Docsify site** — Documentation now renders correctly on GitHub Pages with a styled coverpage.

## Quick start

### 1. Add the dependency

```toml
[dependencies]
dioxus-i18n-json = "0.0.4"
```

For development hot-reload (desktop only):

```toml
[dependencies]
dioxus-i18n-json = { version = "0.0.4", features = ["hot-reload"] }
```

### 2. Create locale files

`locales/en.json`
```json
{
  "greeting": "Hello, {name}!",
  "messages": {
    "welcome": "Welcome to dioxus-i18n-json.",
    "description": "Try editing this file while the app is running."
  },
  "itemCount": {
    "one": "You have {count} item.",
    "other": "You have {count} items."
  },
  "terms": "Read our <0>Terms of Service</0>."
}
```

`locales/es.json`
```json
{
  "greeting": "¡Hola, {name}!",
  "messages": {
    "welcome": "Bienvenido a dioxus-i18n-json.",
    "description": "Prueba a editar este archivo mientras la aplicación está en ejecución."
  },
  "itemCount": {
    "one": "Tienes {count} artículo.",
    "other": "Tienes {count} artículos."
  },
  "terms": "Lee nuestros <0>Términos de Servicio</0>."
}
```

### 3. Wire up the provider

```rust
use dioxus::prelude::*;
use dioxus_i18n_json::{generate_keys, I18nConfig, I18nProvider, Trans, use_i18n, use_t, use_t_ns, UseI18n};

// Generate typed keys at compile time from your reference locale.
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

    // Destructurable translation helpers (React-i18n style)
    let UseI18n { t, tp, tf } = use_t();
    let UseI18n { t: t_msg, .. } = use_t_ns("messages");

    let count = 5i64;

    rsx! {
        // Typed keys give you IDE autocomplete and catch typos at compile time
        h1 { {tf(keys::greeting, &[("name", "Alice")])} }
        p  { {t_msg(keys::messages::welcome)} }
        p  { {t_msg(keys::messages::description)} }
        p  { {tp(keys::itemCount, count)} }

        Trans {
            text: t(keys::terms),
            slots: vec![
                rsx! { a { href: "/tos", "Terms of Service" } }
            ]
        }

        button {
            onclick: move |_| i18n.set_locale("es"),
            "Switch to Spanish"
        }
    }
}
```

## Running the examples

### `hello-i18n` – general desktop demo
```bash
cargo run -p hello-i18n
```
Shows basic usage, locale switching, plurals, formatting, and `Trans`.

### `minimal-i18n` – smallest possible setup
```bash
cargo run -p minimal-i18n
```
A one-key example to see the bare minimum needed to get started.

### `web-i18n` – browser target
```bash
# Serve with the Dioxus CLI (do NOT use `cargo run` for web targets)
dx serve -p web-i18n
```
Demonstrates the same API running in a web app.

### `advanced-i18n` – complex real-world UI
```bash
cargo run -p advanced-i18n
```
Features nested namespaces, multiple `Trans` slots, rich plurals (Russian/Polish `few`/`many` forms), and hot-reload. Try editing `examples/advanced-i18n/locales/*.json` while the app runs — the UI updates instantly.

## How it works

- On startup, `I18nProvider` reads every `*.json` file in the configured directory.
- Nested JSON objects are flattened into dot-notation keys (`messages.welcome`), except objects whose keys are plural categories (`one`, `other`, etc.) which are preserved as plural forms.
- `use_i18n()` returns the full handle backed by two Dioxus signals: `locale` and `translations`.
- `use_t()` and `use_t_ns("ns")` return a destructurable `UseI18n { t, tp, tf }` struct:
  - `t("key")` – simple lookup.
  - `tp("key", count)` – pluralized lookup using CLDR rules; `{count}` is auto-replaced.
  - `tf("key", &[("name", "Alice")])` – formatted lookup with `{name}` placeholders.
- `Trans` parses translation strings for component placeholders like `<0>...</0>` and renders the matching `slots` in their place.
- `generate_keys!("locales/en.json")` reads the JSON at compile time and emits a `keys` module containing the fully-qualified dot-notation strings.
- Calling any translation function reads the underlying signals, so components automatically re-render when either changes.
- When `hot-reload` is enabled, a background file watcher (via `notify`) monitors the locales directory and reloads the translations signal on every change.

## Documentation

Full documentation is available in the `docs/` folder (powered by [Docsify](https://docsify.js.org/)).

To view it locally:

```bash
cd docs
npx docsify serve
```

Or open `docs/index.html` with any static file server.

You can also read the docs online at:  
**https://eftech93.github.io/dioxus-i18n** (after publishing).

### Web / WASM support

Browsers cannot read the local filesystem at runtime, so for web targets you should embed the JSON files at compile time with `include_str!` and use `I18nConfig::embedded()`:

```rust
use std::collections::HashMap;
use dioxus_i18n_json::{I18nConfig, I18nProvider, parse_translation_json};

fn App() -> Element {
    let mut translations = HashMap::new();
    translations.insert(
        "en".to_string(),
        parse_translation_json(include_str!("../locales/en.json")).unwrap(),
    );
    translations.insert(
        "ja".to_string(),
        parse_translation_json(include_str!("../locales/ja.json")).unwrap(),
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
