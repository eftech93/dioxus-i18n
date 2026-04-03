# Plurals & Interpolation

## String interpolation (`tf`)

Use `{name}` placeholders in your JSON and replace them at runtime.

**JSON:**
```json
{
  "greeting": "Hello, {name}!"
}
```

**Rust:**
```rust
let UseI18n { tf, .. } = use_t();
tf(keys::greeting, &[("name", "Alice")]);
// → "Hello, Alice!"
```

You can pass as many variables as you need:

```rust
tf(keys::profile_info, &[
    ("name", "Alice"),
    ("date", "2024-05-12"),
]);
```

## Pluralization (`tp`)

`dioxus-i18n-json` uses Unicode CLDR plural rules to pick the correct form for a given count.

### English example

**JSON:**
```json
{
  "itemCount": {
    "one": "You have {count} item.",
    "other": "You have {count} items."
  }
}
```

**Rust:**
```rust
let UseI18n { tp, .. } = use_t();
tp(keys::itemCount, 1);  // → "You have 1 item."
tp(keys::itemCount, 5);  // → "You have 5 items."
```

`{count}` is replaced automatically.

### Complex languages (Russian, Polish, Arabic)

Some languages have more than two plural forms.

**Russian (`ru`):**
```json
{
  "notifications": {
    "one": "{count} уведомление",
    "few": "{count} уведомления",
    "many": "{count} уведомлений",
    "other": "{count} уведомлений"
  }
}
```

**Rust:**
```rust
let UseI18n { tp, .. } = use_t();
tp(keys::notifications, 1);  // → "1 уведомление"
tp(keys::notifications, 2);  // → "2 уведомления"
tp(keys::notifications, 5);  // → "5 уведомлений"
```

### Supported plural categories

| Category | Typical use |
|----------|-------------|
| `zero`   | 0 in Arabic, Latvian, etc. |
| `one`    | 1 in English, German, etc. |
| `two`    | 2 in Arabic, Welsh, etc. |
| `few`    | 2–4 in Russian, Polish, etc. |
| `many`   | 5+ in Russian, Polish, etc. |
| `other`  | Fallback for all languages |

If a specific category is missing, `other` is used as the fallback.
