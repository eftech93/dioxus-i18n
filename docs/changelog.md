# Changelog

## 0.0.4

### Improvements

- **Robust `generate_keys!` macro** — The proc-macro now safely handles edge-case JSON keys:
  - Keys containing special characters (`-`, `.`, spaces) are sanitized without panicking.
  - Duplicate identifiers caused by different keys mapping to the same Rust name (e.g. `foo-bar` and `foo_bar`) are automatically deduplicated with `_2`, `_3`, etc.
  - Empty or underscore-only keys receive stable hashed fallbacks instead of causing compilation errors.
- Added `.nojekyll` and fixed Docsify configuration for proper GitHub Pages rendering.
- Added a styled coverpage to the documentation.

## 0.0.1

### Initial Release

- JSON-based locale files with dot-notation key lookup.
- `I18nProvider` and `use_i18n()` reactive hooks.
- `use_t()` / `use_t_ns()` destructurable translation helpers.
- Unicode CLDR plural support via `tp()`.
- String interpolation via `tf()`.
- `<Trans>` component for embedding Dioxus components inside translations.
- `generate_keys!` proc-macro for compile-time typed keys.
- Desktop hot-reload support via `notify`.
- Web / WASM support via `I18nConfig::embedded()` and `parse_translation_json()`.
- Four working examples: `minimal-i18n`, `hello-i18n`, `web-i18n`, `advanced-i18n`.
