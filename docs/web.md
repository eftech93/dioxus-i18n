# Web / WASM

Browsers cannot access the local filesystem at runtime, so `I18nConfig::new("./locales", "en")` will not work for web targets. Instead, embed the JSON files directly into the WASM binary at compile time.

## Embedding translations

Use `include_str!` together with `parse_translation_json` and `I18nConfig::embedded`:

```rust
use dioxus::prelude::*;
use dioxus_i18n_json::{I18nConfig, I18nProvider, parse_translation_json};
use std::collections::HashMap;

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

Everything else — `use_t`, `tp`, `tf`, `Trans`, `generate_keys!` — works exactly the same as on desktop.

## Running the web example

> ⚠️ **Do not use `cargo run` for web examples.** Web targets compile to WebAssembly and must be served with the Dioxus CLI.

```bash
cd /Users/tebo1993/Desktop/EFTECH93/dioxus-i18n
dx serve -p web-i18n
```

Then open `http://localhost:8080`.

If you see `cannot call wasm-bindgen imported functions on non-wasm targets`, it means you accidentally ran `cargo run` instead of `dx serve`.

If port 8080 is busy, specify another one:

```bash
dx serve -p web-i18n --port 3000
```
