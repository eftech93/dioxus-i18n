use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
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

    let keys_module = generate_value(&value, "keys", "", &mut HashSet::new());
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

fn generate_value(
    value: &serde_json::Value,
    name: &str,
    prefix: &str,
    used: &mut HashSet<String>,
) -> proc_macro2::TokenStream {
    match value {
        serde_json::Value::Object(obj) => {
            if is_plural_object(obj) {
                let const_ident = safe_ident(name);
                return quote! { pub const #const_ident: &str = #prefix; };
            }

            let ident = safe_ident(name);
            let mut local_used = HashSet::new();
            let items: Vec<_> = obj
                .iter()
                .map(|(k, v)| {
                    let mut sanitized = sanitize_ident(k);
                    if sanitized.is_empty() || sanitized == "_" {
                        sanitized = format!("_{}", hash_key(k));
                    }
                    let full_key = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    if v.is_object() && is_plural_object(v.as_object().unwrap()) {
                        let unique = unique_ident(&mut local_used, &sanitized);
                        let const_ident = safe_ident(&unique);
                        quote! { pub const #const_ident: &str = #full_key; }
                    } else if v.is_object() || v.is_array() {
                        let unique = unique_ident(&mut local_used, &sanitized);
                        generate_value(v, &unique, &full_key, used)
                    } else {
                        let unique = unique_ident(&mut local_used, &sanitized);
                        let const_ident = safe_ident(&unique);
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
            let mut local_used = HashSet::new();
            let items: Vec<_> = arr
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let idx_name = format!("_{}", i);
                    let full_key = format!("{}[{}]", prefix, i);
                    if v.is_object() && is_plural_object(v.as_object().unwrap()) {
                        let unique = unique_ident(&mut local_used, &idx_name);
                        let const_ident = safe_ident(&unique);
                        quote! { pub const #const_ident: &str = #full_key; }
                    } else if v.is_object() || v.is_array() {
                        let unique = unique_ident(&mut local_used, &idx_name);
                        generate_value(v, &unique, &full_key, used)
                    } else {
                        let unique = unique_ident(&mut local_used, &idx_name);
                        let const_ident = safe_ident(&unique);
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

fn unique_ident(used: &mut HashSet<String>, base: &str) -> String {
    if !used.contains(base) {
        used.insert(base.to_string());
        return base.to_string();
    }
    let mut i = 2;
    loop {
        let candidate = format!("{}_{}", base, i);
        if !used.contains(&candidate) {
            used.insert(candidate.clone());
            return candidate;
        }
        i += 1;
    }
}

fn hash_key(key: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut s = DefaultHasher::new();
    key.hash(&mut s);
    s.finish()
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
    // Try as a normal identifier first
    if let Ok(ident) = syn::parse_str::<syn::Ident>(name) {
        // syn::parse_str accepts "_" but Ident::new panics on it.
        // We already filter out "_" before calling safe_ident, but guard anyway.
        if name != "_" {
            return ident;
        }
    }
    if name.is_empty() {
        return syn::Ident::new("_empty", proc_macro2::Span::call_site());
    }
    if name.chars().next().unwrap().is_ascii_digit() {
        return syn::Ident::new(
            &format!("_{}", name),
            proc_macro2::Span::call_site(),
        );
    }
    syn::Ident::new_raw(name, proc_macro2::Span::call_site())
}
