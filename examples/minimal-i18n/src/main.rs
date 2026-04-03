use dioxus::prelude::*;
use dioxus_i18n_json::{generate_keys, use_t, I18nConfig, I18nProvider, UseI18n};

generate_keys!("examples/minimal-i18n/locales/en.json");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        I18nProvider {
            config: I18nConfig::new("examples/minimal-i18n/locales", "en"),
            Hello {}
        }
    }
}

#[component]
fn Hello() -> Element {
    let UseI18n { t, .. } = use_t();
    rsx! {
        h1 { {t(keys::hello)} }
    }
}
