# Typed Keys

The `generate_keys!` proc-macro turns your JSON locale file into a Rust module full of string constants. This gives you **IDE autocomplete** and **compile-time validation**.

## Generating keys

Add this once at the top of your `main.rs` or `lib.rs`:

```rust
dioxus_i18n_json::generate_keys!("locales/en.json");
```

## Using keys

```rust
let UseI18n { t, .. } = use_t();
t(keys::messages::welcome);
```

If you rename or delete `messages.welcome` in the JSON, the Rust compiler will error instead of silently showing a missing translation at runtime.

## What gets generated?

Given this JSON:

```json
{
  "greeting": "Hello!",
  "messages": {
    "welcome": "Welcome",
    "description": "A description"
  },
  "itemCount": {
    "one": "1 item",
    "other": "{count} items"
  }
}
```

`generate_keys!` emits:

```rust
pub mod keys {
    pub const greeting: &str = "greeting";
    pub mod messages {
        pub const welcome: &str = "messages.welcome";
        pub const description: &str = "messages.description";
    }
    pub const itemCount: &str = "itemCount";
}
```

Note that plural objects are collapsed to a single constant because the key itself (`itemCount`) is what you pass to `tp(...)`.

## Namespaces

Typed keys work seamlessly with `use_t_ns`:

```rust
let UseI18n { t, .. } = use_t_ns("messages");
t(keys::messages::welcome); // still valid
```

But remember: `use_t_ns` auto-prefixes the key, so if you already have the fully-qualified constant, you usually want `use_t()` instead.

## Edge cases handled automatically

The macro is designed to be resilient against real-world JSON keys:

- **Special characters** — Keys like `foo-bar`, `foo.bar`, or `foo bar` are sanitized to valid Rust identifiers.
- **Duplicates** — If two different keys normalize to the same identifier (e.g. `foo-bar` and `foo_bar` both become `foo_bar`), the second one is renamed `foo_bar_2`, the third `foo_bar_3`, etc.
- **Empty or underscore-only keys** — These receive a stable hashed fallback name so the macro never panics.

## Zero runtime cost

The macro only generates `&str` constants. The final binary is identical to using raw string literals.
