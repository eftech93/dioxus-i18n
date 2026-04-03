# Getting Started

This guide walks you through adding `dioxus-i18n-json` to a new or existing Dioxus project.

## 1. Add the dependency

```toml
[dependencies]
dioxus = "0.6"
dioxus-i18n-json = "0.0.3"
```

If you want hot-reload on desktop:

```toml
[dependencies]
dioxus-i18n-json = { version = "0.0.3", features = ["hot-reload"] }
```

## 2. Create locale files

Create a `locales/` directory next to your `Cargo.toml` (or wherever you prefer).

**`locales/en.json`**
```json
{
  "greeting": "Hello, world!",
  "messages": {
    "welcome": "Welcome to dioxus-i18n-json."
  }
}
```

**`locales/es.json`**
```json
{
  "greeting": "¡Hola, mundo!",
  "messages": {
    "welcome": "Bienvenido a dioxus-i18n-json."
  }
}
```

## 3. Wire up the provider

Wrap your app (or the part that needs translations) with `I18nProvider`:

```rust
use dioxus::prelude::*;
use dioxus_i18n_json::{I18nConfig, I18nProvider};

fn App() -> Element {
    rsx! {
        I18nProvider {
            config: I18nConfig::new("./locales", "en"),
            AppContent {}
        }
    }
}
```

`I18nConfig::new(locales_dir, default_locale)` tells the library where to find JSON files and which locale to start with.

## 4. Translate in components

Use `use_i18n()` or `use_t()` inside any child component:

```rust
use dioxus::prelude::*;
use dioxus_i18n_json::{use_i18n, use_t, UseI18n};

#[component]
fn AppContent() -> Element {
    let mut i18n = use_i18n();
    let UseI18n { t, .. } = use_t();

    rsx! {
        h1 { {t("greeting")} }
        p  { {t("messages.welcome")} }

        button {
            onclick: move |_| i18n.set_locale("es"),
            "Switch to Spanish"
        }
    }
}
```

When the locale changes, every component that read from i18n will automatically re-render.

## 5. (Optional) Add typed keys

Call `generate_keys!` once in your crate root to get compile-time constants:

```rust
dioxus_i18n_json::generate_keys!("locales/en.json");
```

Then write:

```rust
let UseI18n { t, .. } = use_t();
t(keys::messages::welcome)
```

This gives you IDE autocomplete and prevents typos at compile time.

## What's next?

- [API Reference](api.md)
- [Plurals & Interpolation](plurals.md)
- [Component Wrappers](trans.md)
