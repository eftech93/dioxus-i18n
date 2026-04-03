# API Reference

## Core Types

### `I18nConfig`

Configuration for the i18n provider.

```rust
pub struct I18nConfig {
    pub locales_dir: PathBuf,
    pub default_locale: String,
    pub initial: Option<HashMap<String, Translations>>,
}
```

#### Constructors

- `I18nConfig::new(locales_dir, default_locale)` — loads JSON files from disk.
- `I18nConfig::embedded(default_locale, translations)` — uses pre-loaded translations (great for WASM).

---

### `I18nProvider`

A component that loads translations and provides the i18n context.

```rust
#[component]
pub fn I18nProvider(children: Element, config: I18nConfig) -> Element
```

Example:

```rust
rsx! {
    I18nProvider {
        config: I18nConfig::new("./locales", "en"),
        AppContent {}
    }
}
```

---

### `I18n`

The handle returned by `use_i18n()`.

```rust
pub struct I18n { /* ... */ }
```

Methods:

- `t(&self, key: &str) -> String` — simple translation lookup.
- `tp(&self, key: &str, count: i64) -> String` — pluralized lookup; `{count}` is auto-replaced.
- `tf(&self, key: &str, vars: &[(&str, &str)]) -> String` — formatted lookup with `{name}` placeholders.
- `set_locale(&mut self, locale: &str)` — change the active locale.
- `locale(&self) -> String` — get the current locale code.

---

### `use_i18n`

Returns the full `I18n` handle. Must be called inside a tree wrapped by `I18nProvider`.

```rust
pub fn use_i18n() -> I18n
```

---

### `UseI18n`

A destructurable struct returned by `use_t()` and `use_t_ns()`.

```rust
pub struct UseI18n<T, TP, TF> {
    pub t: T,
    pub tp: TP,
    pub tf: TF,
}
```

---

### `use_t`

Returns translation helpers with fully-qualified keys.

```rust
pub fn use_t() -> UseI18n<impl Fn(&str) -> String, impl Fn(&str, i64) -> String, impl Fn(&str, &[(&str, &str)]) -> String>
```

Example:

```rust
let UseI18n { t, tp, tf } = use_t();
t("messages.welcome");
tp("itemCount", 5);
tf("greeting", &[("name", "Alice")]);
```

---

### `use_t_ns`

Returns translation helpers bound to a namespace (auto-prefixes keys).

```rust
pub fn use_t_ns(ns: &'static str) -> UseI18n<...>
```

Example:

```rust
let UseI18n { t, .. } = use_t_ns("messages");
t("welcome"); // resolves to "messages.welcome"
```

---

### `Trans`

A component that renders a translation string containing component placeholders (`<0>...</0>`).

```rust
#[component]
pub fn Trans(text: String, slots: Vec<Element>) -> Element
```

Example:

```rust
Trans {
    text: t("terms"),
    slots: vec![
        rsx! { a { href: "/tos", "Terms of Service" } }
    ]
}
```

---

### `generate_keys!`

Proc-macro that generates a `keys` module from a JSON locale file.

```rust
dioxus_i18n_json::generate_keys!("locales/en.json");
```

After calling it, you can use constants like:

```rust
keys::greeting
keys::messages::welcome
```

---

### `parse_translation_json`

Parses a JSON string into the internal `Translations` map.

```rust
pub fn parse_translation_json(json: &str) -> Result<Translations, LoadError>
```

Useful for embedding translations with `include_str!` on WASM.
