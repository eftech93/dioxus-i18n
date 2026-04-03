use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Generate a `keys` module containing typed translation keys from a JSON locale file.
///
/// # Example
/// ```rust,ignore
/// dioxus_i18n_json::generate_keys!("locales/en.json");
///
/// let UseI18n { t } = use_t();
/// t(keys::messages::welcome);
/// ```
#[proc_macro]
pub fn generate_keys(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr).value();
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("could not read locale file '{}': {}", path, e),
            )
            .to_compile_error()
            .into();
        }
    };
    let value: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("invalid JSON in '{}': {}", path, e),
            )
            .to_compile_error()
            .into();
        }
    };

    let keys_module = generate_value(&value, "keys", "");
    quote! {
        #keys_module
    }
    .into()
}

fn is_plural_object(obj: &serde_json::Map<String, serde_json::Value>) -> bool {
    if obj.is_empty() {
        return false;
    }
    obj.keys().all(|k| {
        matches!(
            k.as_str(),
            "zero" | "one" | "two" | "few" | "many" | "other"
        )
    })
}

fn generate_value(value: &serde_json::Value, name: &str, prefix: &str) -> proc_macro2::TokenStream {
    match value {
        serde_json::Value::Object(obj) => {
            if is_plural_object(obj) {
                let const_ident = safe_ident(name);
                return quote! { pub const #const_ident: &str = #prefix; };
            }

            let ident = safe_ident(name);
            let items: Vec<_> = obj
                .iter()
                .map(|(k, v)| {
                    let sanitized = sanitize_ident(k);
                    let full_key = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    if v.is_object() && is_plural_object(v.as_object().unwrap()) {
                        let const_ident = safe_ident(&sanitized);
                        quote! { pub const #const_ident: &str = #full_key; }
                    } else if v.is_object() || v.is_array() {
                        generate_value(v, &sanitized, &full_key)
                    } else {
                        let const_ident = safe_ident(&sanitized);
                        quote! { pub const #const_ident: &str = #full_key; }
                    }
                })
                .collect();
            quote! {
                pub mod #ident {
                    #(#items)*
                }
            }
        }
        serde_json::Value::Array(arr) => {
            let ident = safe_ident(name);
            let items: Vec<_> = arr
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let idx_name = format!("_{}", i);
                    let full_key = format!("{}[{}]", prefix, i);
                    if v.is_object() && is_plural_object(v.as_object().unwrap()) {
                        let const_ident = safe_ident(&idx_name);
                        quote! { pub const #const_ident: &str = #full_key; }
                    } else if v.is_object() || v.is_array() {
                        generate_value(v, &idx_name, &full_key)
                    } else {
                        let const_ident = safe_ident(&idx_name);
                        quote! { pub const #const_ident: &str = #full_key; }
                    }
                })
                .collect();
            quote! {
                pub mod #ident {
                    #(#items)*
                }
            }
        }
        _ => {
            // Scalar at root level (shouldn't happen with named root, but handle gracefully)
            let const_ident = safe_ident(name);
            quote! { pub const #const_ident: &str = #prefix; }
        }
    }
}

fn sanitize_ident(raw: &str) -> String {
    raw.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn safe_ident(name: &str) -> syn::Ident {
    if syn::parse_str::<syn::Ident>(name).is_ok() {
        syn::Ident::new(name, proc_macro2::Span::call_site())
    } else if !name.is_empty() && name.chars().next().unwrap().is_ascii_digit() {
        syn::Ident::new(&format!("_{}", name), proc_macro2::Span::call_site())
    } else {
        syn::Ident::new_raw(name, proc_macro2::Span::call_site())
    }
}
