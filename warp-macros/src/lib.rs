extern crate proc_macro2;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, FnArg, GenericArgument, Ident, ItemFn,
    Meta, Pat, PathArguments, ReturnType, Signature, Type,
};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn c_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr with Punctuated<Meta, syn::Token![,]>::parse_terminated);
    let asyn = Ident::new("asyn", Span::call_site());
    let is_async = attr.iter().any(|a| a.path().get_ident() == Some(&asyn));
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;

    let vec_reg = regex::Regex::new(r"Vec < (\w+) >").unwrap();
    let Signature {
        ident,
        inputs,
        output,
        ..
    } = sig;

    let ReturnType::Type(_, retype) = output else {
        abort!(output, "1")
    };
    let Type::Path(type_path) = retype.as_ref() else {
        abort!(retype, "2")
    };
    let path = &type_path.path;
    assert_eq!(path.segments[0].ident.to_string(), "Result");
    let PathArguments::AngleBracketed(type_arg) = &path.segments[0].arguments else {
        abort!(path, "3")
    };
    let GenericArgument::Type(result_type) = &type_arg.args[0] else {
        abort!(type_arg, "4")
    };
    let mut c_result_type = result_type.clone();
    let result_type_str = quote! { #result_type }.to_string();
    let map_result = match result_type_str.as_str() {
        "()" => {
            c_result_type = parse_quote! {
                u8
            };
            quote! {
                let res = res.map(|_| 0u8);
                map_result(res)
            }
        }
        "String" => quote! {
            map_result_string(res)
        },
        "Vec < u8 >" => {
            c_result_type = parse_quote! {
                *const u8
            };
            quote! {
                map_result_bytes(res)
            }
        }
        _ => {
            if let Some(c) = vec_reg.captures(&result_type_str) {
                let ty = &c[1];
                let ty = Ident::new(ty, Span::call_site());
                c_result_type = parse_quote! {
                    *const u8
                };
                quote! {
                    let data = res.map(|res| {
                        let mut builder = FlatBufferBuilder::new();
                        let mut os = Vec::new();
                        for v in res.iter() {
                            let o = v.pack(&mut builder);
                            builder.push(o);
                            os.push(o);
                        }
                        builder.start_vector::<WIPOffset<#ty>>(res.len());
                        for o in os {
                            builder.push(o);
                        }
                        let o = builder.end_vector::<WIPOffset<#ty>>(res.len());
                        builder.finish(o, None);
                        let data = builder.finished_data();
                        data.to_vec()
                    });
                    map_result_bytes(data)
                }
            } else if result_type_str.ends_with("T") {
                c_result_type = parse_quote! {
                    *const u8
                };
                quote! {
                    let data = res.map(|res| {
                        let mut builder = FlatBufferBuilder::new();
                        let ret_bytes = res.pack(&mut builder);
                        builder.finish(ret_bytes, None);
                        let data = builder.finished_data().to_vec();
                        data
                    });
                    map_result_bytes(data)
                }
            } else {
                quote! {
                    map_result(res)
                }
            }
        }
    };

    let ItemFn { vis, block, .. } = &input;
    let mut wrapped_fnargs = vec![];
    let mut str_convs = vec![];
    let mut bytes_convs = vec![];
    let mut has_network = false;
    let mut has_connection = false;
    let mut has_client = false;
    for input in inputs.iter() {
        if let FnArg::Typed(pat) = input {
            if let Pat::Ident(param) = pat.pat.as_ref() {
                let ident = &param.ident;
                let name = ident.to_string();
                match name.as_str() {
                    "network" => {
                        has_network = true;
                        continue;
                    }
                    "connection" => {
                        has_connection = true;
                        continue;
                    }
                    "client" if is_async => {
                        has_client = true;
                        continue;
                    }
                    _ => {}
                }
                let p = pat.ty.as_ref();
                let s = quote! { #p }.to_string();
                match s.as_str() {
                    "& str" => {
                        str_convs.push(quote! {
                            let #ident = unsafe { CStr::from_ptr(#ident).to_string_lossy() };
                            let #ident = &#ident;
                        });
                        let input: FnArg = parse_quote! {
                            #ident: *mut c_char
                        };
                        wrapped_fnargs.push(input);
                        continue;
                    }
                    "& [u8]" => {
                        bytes_convs.push(quote! {
                            let #ident = unsafe {
                                let ptr_len = #ident as *mut u32;
                                let len = *ptr_len as usize;
                                let ptr_data = value.offset(4);
                                Vec::<u8>::from_raw_parts(ptr_data, len, len)
                            };
                            let #ident = &#ident[..];
                        });
                        let input: FnArg = parse_quote! {
                            #ident: *mut u8
                        };
                        wrapped_fnargs.push(input);
                        continue;
                    }
                    _ => {
                    }
                }
            }
        }
        wrapped_fnargs.push(input.clone());
    }

    let network = if has_network {
        quote! {
            let network = &coin.network;
        }
    } else {
        quote! {}
    };
    let connection = if has_connection {
        quote! {
            let connection = &coin.connection()?;
        }
    } else {
        quote! {}
    };
    let client = if has_client {
        quote! {
            let mut client = coin.connect_lwd().await?;
            let client = &mut client;
        }
    } else {
        quote! {}
    };

    let args = inputs
        .iter()
        .map(|arg| {
            let FnArg::Typed(pat_type) = arg else {
                abort!(arg, "fnarg::pat_type");
            };
            let syn::Pat::Ident(pat_ident) = &*pat_type.pat else {
                abort!(pat_type, "fnarg::pat_ident");
            };
            &pat_ident.ident
        })
        .collect::<Vec<_>>();

    let wrapper = Ident::new(&format!("c_{}", ident), ident.span());

    let res = if is_async {
        quote! {
            #[no_mangle]
            #[tokio::main]
            pub async extern "C" fn #wrapper(coin: u8, #(#wrapped_fnargs),*) -> CResult<#c_result_type> {
                let res = async {
                    let coin = COINS[coin as usize].lock();
                    #network
                    #connection
                    #client
                    #(#str_convs),*
                    #(#bytes_convs),*
                    #ident(#(#args),*).await
                };
                let res = res.await;
                #map_result
            }

            #vis async fn #ident(#inputs) #output #block
        }
    } else {
        quote! {
            #[no_mangle]
            pub extern "C" fn #wrapper(coin: u8, #(#wrapped_fnargs),*) -> CResult<#c_result_type> {
                let res = || {
                    let coin = COINS[coin as usize].lock();
                    #network
                    #connection
                    #client
                    #(#str_convs),*
                    #(#bytes_convs),*
                    #ident(#(#args),*)
                };
                let res = res();
                #map_result
            }

            #vis fn #ident(#inputs) #output #block

        }
    };

    res.into()
}
