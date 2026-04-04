# dioxus-i18n-json-macro

Proc-macro companion crate for [`dioxus-i18n-json`](https://crates.io/crates/dioxus-i18n-json). It provides the `generate_keys!` macro that reads a JSON locale file at compile time and emits a typed `keys` module, giving you IDE autocomplete and compile-time validation for translation keys.

## Usage

You typically do **not** need to depend on this crate directly. It is re-exported from `dioxus-i18n-json`:

```rust
use dioxus_i18n_json::generate_keys;

generate_keys!("locales/en.json");

fn Example() {
    // keys::messages::welcome is a &'static str equal to "messages.welcome"
    println!("{}", keys::messages::welcome);
}
```

## What it generates

For a JSON file like:

```json
{
  "greeting": "Hello!",
  "messages": {
    "welcome": "Welcome!"
  },
  "itemCount": {
    "one": "One item",
    "other": "Many items"
  }
}
```

The macro generates:

```rust
pub mod keys {
    pub const greeting: &str = "greeting";
    pub mod messages {
        pub const welcome: &str = "messages.welcome";
    }
    pub mod itemCount {
        pub const one: &str = "itemCount";
        pub const other: &str = "itemCount";
    }
}
```

Keys whose values are objects containing only CLDR plural categories (`zero`, `one`, `two`, `few`, `many`, `other`) are emitted as flat constants pointing to the parent dot-notation key, because plural selection is handled at runtime.

## Edge cases

The macro safely handles:

- Special characters in JSON keys (`-`, `.`, spaces) – sanitized to valid Rust identifiers.
- Empty or underscore-only keys – hashed to unique identifiers.
- Duplicate identifiers after sanitization – deduplicated with numeric suffixes.
- Raw identifiers for Rust keywords.

## License

MIT OR Apache-2.0

Repository: [https://github.com/eftech93/dioxus-i18n](https://github.com/eftech93/dioxus-i18n)
