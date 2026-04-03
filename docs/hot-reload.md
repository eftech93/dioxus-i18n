# Hot Reload

During development it is convenient to edit locale files and see changes instantly without restarting the app. `dioxus-i18n-json` supports this via the `hot-reload` feature.

> ⚠️ Hot-reload requires a filesystem watcher and therefore only works on **desktop** targets. On **web / WASM**, use `dx serve`'s asset reload or rebuild the app.

## Enabling the feature

Add the feature to your `Cargo.toml`:

```toml
[dependencies]
dioxus-i18n-json = { version = "0.0.2", features = ["hot-reload"] }
```

## How it works

When `I18nProvider` mounts, it spawns a background file watcher (via `notify`) on the `locales_dir`. Any change detected inside that directory causes the library to re-read all `*.json` files and update the reactive `translations` signal.

Because Dioxus signals are reactive, every component that previously called `t(...)`, `tp(...)`, or `tf(...)` will automatically re-render with the new text.

## Try it

Run the `advanced-i18n` example:

```bash
cargo run -p advanced-i18n
```

While the app is running, open `examples/advanced-i18n/locales/en.json` in your editor, change a string, and save. The window should update within a fraction of a second.

## Disabling in release builds

It is good practice to enable `hot-reload` only for development. You can use a Cargo feature flag in your own app:

```toml
[features]
default = []
dev = ["dioxus-i18n-json/hot-reload"]
```

Then run with `cargo run --features dev` during development and without it for production.
