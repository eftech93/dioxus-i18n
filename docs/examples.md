# Examples

The workspace contains four examples. Each lives in `examples/` and can be run independently.

## `minimal-i18n`

The smallest possible setup — one key, one component.

```bash
cargo run -p minimal-i18n
```

**Highlights:**
- `I18nProvider`
- `use_t()`
- `generate_keys!`

---

## `hello-i18n`

The original desktop demo covering all core features.

```bash
cargo run -p hello-i18n
```

**Highlights:**
- Locale switching (EN / ES / FR)
- String interpolation (`tf`)
- Plurals (`tp`)
- `Trans` component wrapper
- Hot-reload (`--features hot-reload`)

---

## `web-i18n`

A browser-targeted example using `include_str!` to embed translations.

> ⚠️ **Do not use `cargo run` for web examples.** You must use the Dioxus CLI so the app compiles to WebAssembly.

```bash
dx serve -p web-i18n
```

**Highlights:**
- `I18nConfig::embedded()`
- `parse_translation_json()`
- Same API running on WASM

---

## `advanced-i18n`

A realistic UI with complex plurals and multiple component slots.

```bash
cargo run -p advanced-i18n
```

**Highlights:**
- Nested namespaces
- Multiple `<Trans>` slots
- CLDR plurals for Russian (`few` / `many`) and Polish
- Stateful counters to demo live plural changes
- Hot-reload enabled by default
