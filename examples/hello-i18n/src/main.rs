use dioxus::prelude::*;
use dioxus_i18n_json::{
    generate_keys, use_i18n, use_t, use_t_ns, I18nConfig, I18nProvider, Trans, UseI18n,
};

// Generate typed keys at compile time from the reference locale file.
generate_keys!("examples/hello-i18n/locales/en.json");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        I18nProvider {
            config: I18nConfig::new("examples/hello-i18n/locales", "en")
                .with_fallback_locale("en"),
            AppContent {}
        }
    }
}

#[component]
fn AppContent() -> Element {
    let mut i18n = use_i18n();
    let locale = i18n.locale();

    // Destructurable translation helpers
    let UseI18n { t, tp, tf } = use_t();
    let UseI18n { t: t_msg, .. } = use_t_ns("messages");

    // Local state for the plural demo
    let mut count = use_signal(|| 1i64);

    rsx! {
        div {
            style: "font-family: sans-serif; padding: 2rem;",
            h1 { "dioxus-i18n-json example" }
            p { "Current locale: {locale}" }
            hr {}

            h2 { {tf(keys::greeting, &[("name", "Alice")])} }
            p { {t_msg(keys::messages::welcome)} }
            p { {t_msg(keys::messages::description)} }

            hr {}

            div {
                style: "display: flex; align-items: center; gap: 1rem;",
                button {
                    onclick: move |_| count -= 1,
                    "−"
                }
                span { "{count}" }
                button {
                    onclick: move |_| count += 1,
                    "+"
                }
                span {
                    style: "margin-left: 1rem;",
                    {tp(keys::itemCount, count())}
                }
            }

            hr {}

            Trans {
                text: t(keys::terms),
                slots: vec![
                    rsx! { a { href: "#", style: "color: blue;", "Terms of Service" } }
                ]
            }

            hr {}

            div {
                style: "display: flex; gap: 1rem;",
                button {
                    onclick: move |_| i18n.set_locale("en"),
                    "🇬🇧 English"
                }
                button {
                    onclick: move |_| i18n.set_locale("es"),
                    "🇪🇸 Español"
                }
                button {
                    onclick: move |_| i18n.set_locale("fr"),
                    "🇫🇷 Français"
                }
            }
        }
    }
}
