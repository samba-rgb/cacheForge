use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, FnArg, PatType, ReturnType};

#[proc_macro_attribute]
pub fn cachable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    if let Item::Fn(input_fn) = input {
        let fn_name = &input_fn.sig.ident;
        let fn_args = &input_fn.sig.inputs;
        let fn_return_type = match &input_fn.sig.output {
            ReturnType::Type(_, ty) => ty.clone(),
            ReturnType::Default => {
                return TokenStream::from(quote! {
                    compile_error!("Functions with #[cachable] must have a return type.");
                });
            }
        };
        let fn_body = &input_fn.block;

        // Generate cache key based on argument names
        let arg_names: Vec<_> = fn_args
            .iter()
            .filter_map(|arg| {
                if let FnArg::Typed(PatType { pat, .. }) = arg {
                    Some(quote! { #pat })
                } else {
                    None
                }
            })
            .collect();

        let generate_key = if arg_names.is_empty() {
            quote! {
                let key = String::from("static_key");
            }
        } else {
            quote! {
                let key = format!("{:?}", (#(#arg_names),*));
            }
        };

        // Unique cache variable for this function
        let cache_name = quote::format_ident!("{}_CACHE", fn_name.to_string().to_uppercase());

        // Generate the expanded function
        let expanded = quote! {
            ::lazy_static::lazy_static! {
                static ref #cache_name: ::std::sync::Mutex<std::collections::HashMap<String, #fn_return_type>> =
                    ::std::sync::Mutex::new(std::collections::HashMap::new());
            }

            fn #fn_name(#fn_args) -> #fn_return_type {
                #generate_key

                // Check the cache
                {
                    let mut cache = #cache_name.lock().unwrap();
                    if let Some(cached) = cache.get(&key) {
                        return cached.clone();
                    }
                }

                // Compute the result
                let result = (|| #fn_body)();

                // Store in the cache
                {
                    let mut cache = #cache_name.lock().unwrap();
                    cache.insert(key, result.clone());
                }

                result
            }
        };

        TokenStream::from(expanded)
    } else {
        TokenStream::from(quote! {
            compile_error!("The #[cachable] attribute can only be used on functions.");
        })
    }
}
