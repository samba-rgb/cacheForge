use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, FnArg, PatType, ReturnType, Lit, Meta, Expr};

#[proc_macro_attribute]
pub fn lru_cache(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute to extract the cache size
    let cache_size = if !attr.is_empty() {
        let parsed_attr = parse_macro_input!(attr as Meta);
        match parsed_attr {
            Meta::NameValue(name_value) => {
                // Accessing the value as a reference to Expr
                if let Expr::Lit(expr_lit) = name_value.value {
                    if let Lit::Int(lit_int) = expr_lit.lit {
                        lit_int.base10_parse::<usize>().unwrap_or(2) // Default to 2 if parsing fails
                    } else {
                        2
                    }
                } else {
                    2 // Default to 2 if the value isn't a literal expression
                }
            }
            _ => 2, // Default to 2 if no valid size is provided
        }
    } else {
        2 // Default size if no attribute is provided
    };

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
                static ref #cache_name: ::std::sync::Mutex<::cacheForge::LruCache<String, #fn_return_type>> =
                    ::std::sync::Mutex::new(::cacheForge::LruCache::new(#cache_size));
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


#[proc_macro_attribute]
pub fn cachable(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute to extract the cache size
    let cache_size = if !attr.is_empty() {
        let parsed_attr = parse_macro_input!(attr as Meta);
        match parsed_attr {
            Meta::NameValue(name_value) => {
                // Accessing the value as a reference to Expr
                if let Expr::Lit(expr_lit) = name_value.value {
                    if let Lit::Int(lit_int) = expr_lit.lit {
                        lit_int.base10_parse::<usize>().unwrap_or(2) // Default to 2 if parsing fails
                    } else {
                        2 // Default to 2 if the value isn't an integer literal
                    }
                } else {
                    2 // Default to 2 if the value isn't a literal expression
                }
            }
            _ => 2, // Default to 2 if no valid size is provided
        }
    } else {
        2 // Default size if no attribute is provided
    };

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
                static ref #cache_name: ::std::sync::Mutex<::cacheForge::LruCache<String, #fn_return_type>> =
                    ::std::sync::Mutex::new(::cacheForge::LruCache::new(#cache_size));
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


#[proc_macro_attribute]
pub fn expire_cache(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute to extract the cache size
    let expire_time = if !attr.is_empty() {
        let parsed_attr = parse_macro_input!(attr as Meta);
        match parsed_attr {
            Meta::NameValue(name_value) => {
                // Accessing the value as a reference to Expr
                if let Expr::Lit(expr_lit) = name_value.value {
                    if let Lit::Int(lit_int) = expr_lit.lit {
                        lit_int.base10_parse::<usize>().unwrap_or(2) // Default to 2 if parsing fails
                    } else {
                        2 // Default to 2 if the value isn't an integer literal
                    }
                } else {
                    2 // Default to 2 if the value isn't a literal expression
                }
            }
            _ => 2, // Default to 2 if no valid size is provided
        }
    } else {
        2 // Default size if no attribute is provided
    };

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
                static ref #cache_name: ::std::sync::Mutex<::cacheForge::ExpireCache<String, #fn_return_type>> =
                    ::std::sync::Mutex::new(::cacheForge::ExpireCache::new());
            }

            fn #fn_name(#fn_args) -> #fn_return_type {
                #generate_key

                {
                    let mut cache = #cache_name.lock().unwrap();
                    if let Some(cached) = cache.get(&key) {
                        return cached.clone();
                    }
                }

                let result = (|| #fn_body)();

                {
                    let mut cache = #cache_name.lock().unwrap();
                    cache.insert(key, result.clone(), #expire_time);
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
