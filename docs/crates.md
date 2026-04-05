# Workspace Crates

`dioxus-i18n` is organized as a Cargo workspace with two published crates. Each crate has a distinct responsibility.

---

## `dioxus-i18n-json`

The **main library** that you add to your `Cargo.toml`.

```toml
[dependencies]
dioxus-i18n-json = "0.0.5"
```

### What it provides

- **`I18nProvider`** — Loads translations and provides the i18n context to your Dioxus app.
- **`use_i18n`**, **`use_t`**, **`use_t_ns`** — React-style hooks for reading and switching translations.
- **`Trans`** — Component wrapper for embedding Dioxus elements (links, bold text, etc.) inside translation strings.
- **`I18nConfig`** — Configuration for filesystem loading (`new`) or embedded translations (`embedded`).
- **Hot-reload** — File-watcher support behind the `hot-reload` feature flag (desktop only).

### Re-export

The main crate also re-exports `generate_keys!` from its companion proc-macro crate, so you only need one dependency in most cases:

```rust
use dioxus_i18n_json::generate_keys;
```

---

## `dioxus-i18n-json-macro`

The **proc-macro companion crate** that powers `generate_keys!`.

```toml
[dependencies]
dioxus-i18n-json-macro = "0.0.5"
```

> **You usually do not need this directly.** It is re-exported by `dioxus-i18n-json`.

### What it provides

- **`generate_keys!`** — A compile-time macro that reads a JSON locale file and emits a `keys` module full of `&str` constants. This gives you IDE autocomplete and compile-time validation for translation keys.

### How it works

The macro:
1. Reads the specified JSON file at compile time.
2. Flattens nested objects into dot-notation keys.
3. Generates a public module tree where each leaf is a `&str` constant equal to the full dot-notation path.
4. Sanitizes special characters, deduplicates collisions, and handles edge-case identifiers safely.

Because the output is only `&str` constants, there is **zero runtime cost**.

---

## Which crate should you depend on?

| Use case | Dependency |
|----------|------------|
| Building a Dioxus app with i18n | `dioxus-i18n-json` |
| Just need the `generate_keys!` macro in another crate | `dioxus-i18n-json-macro` |

Both crates are versioned in lock-step from the workspace root, so their versions always match.
