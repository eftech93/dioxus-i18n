# Component Wrappers (`Trans`)

Sometimes a translation needs to contain interactive elements: links, bold text, icons, etc.
`Trans` lets you embed Dioxus components inside translation strings using numbered tags.

## Syntax

In your JSON, use `<0>...</0>`, `<1>...</1>`, etc. The text between the tags is ignored at runtime; the matching slot from `slots` is rendered instead.

**JSON:**
```json
{
  "terms": "Read our <0>Terms of Service</0>.",
  "subtitle": "Built with <0>dioxus-i18n-json</0> and <1>Rust</1>."
}
```

## Single slot

```rust
Trans {
    text: t(keys::terms),
    slots: vec![
        rsx! { a { href: "/tos", "Terms of Service" } }
    ]
}
```

Renders:  
`Read our `[Terms of Service](#)`.`

## Multiple slots

```rust
Trans {
    text: t(keys::subtitle),
    slots: vec![
        rsx! { a { href: "https://github.com/eftech93/dioxus-i18n", "dioxus-i18n-json" } },
        rsx! { a { href: "https://www.rust-lang.org", "Rust" } }
    ]
}
```

Renders:  
`Built with `[dioxus-i18n-json](https://github.com/eftech93/dioxus-i18n)` and `[Rust](https://www.rust-lang.org)`.`

## Combining with plurals

You can also use `Trans` with pluralized strings:

```rust
Trans {
    text: tp(keys::unread, count),
    slots: vec![
        rsx! { strong { style: "color: red;", "" } }
    ]
}
```

**JSON:**
```json
{
  "unread": {
    "one": "You have <0>{count} unread message</0>.",
    "other": "You have <0>{count} unread messages</0>."
  }
}
```

This renders the count inside a bold `<strong>` element.

## Nested tags

`Trans` does a simple linear scan. Tags must be well-formed and not overlap. Nested placeholders like `<0><1>...</1></0>` are not supported.
