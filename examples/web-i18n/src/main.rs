use dioxus::prelude::*;
use dioxus_i18n_json::{generate_keys, I18nConfig, I18nProvider, parse_translation_json, use_i18n, use_t, UseI18n};
use std::collections::HashMap;

generate_keys!("examples/web-i18n/locales/en.json");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Embed locale files at compile time so they work in the browser (WASM).
    let mut translations = HashMap::new();
    translations.insert(
        "en".to_string(),
        parse_translation_json(include_str!("../locales/en.json")).expect("valid en.json"),
    );
    translations.insert(
        "ja".to_string(),
        parse_translation_json(include_str!("../locales/ja.json")).expect("valid ja.json"),
    );

    rsx! {
        I18nProvider {
            config: I18nConfig::embedded("en", translations),
            Page {}
        }
    }
}

#[component]
fn Page() -> Element {
    let mut i18n = use_i18n();
    let UseI18n { tf, .. } = use_t();
    let UseI18n { t, .. } = use_t();
    let lang_label = if i18n.locale() == "ja" {
        t(keys::switch::ja)
    } else {
        t(keys::switch::en)
    };

    rsx! {
        div {
            style: "font-family: sans-serif; padding: 2rem; max-width: 600px; margin: 0 auto;",
            h1 { {tf(keys::title, &[])} }
            p { {tf(keys::intro, &[])} }
            hr {}
            p { {tf(keys::switch::label, &[("lang", &lang_label)])} }
            div {
                style: "display: flex; gap: 1rem; margin-top: 1rem;",
                button {
                    onclick: move |_| i18n.set_locale("en"),
                    "English"
                }
                button {
                    onclick: move |_| i18n.set_locale("ja"),
                    "日本語"
                }
            }
        }
    }
}
