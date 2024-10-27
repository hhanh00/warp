extern crate proc_macro2;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, FnArg, GenericArgument, Ident, ItemFn, Pat, PathArguments,
    ReturnType, Signature, Type,
};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn c_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;
    let is_async = sig.asyncness.is_some();

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
                crate::ffi::map_result(res)
            }
        }
        "String" => {
            c_result_type = parse_quote! {
                *mut std::ffi::c_char
            };
            quote! {
                crate::ffi::map_result_string(res)
            }
        }
        "Vec < u8 >" => {
            c_result_type = parse_quote! {
                *const u8
            };
            quote! {
                crate::ffi::map_result_bytes(res)
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
                        let mut builder = flatbuffers::FlatBufferBuilder::new();
                        let mut os = Vec::new();
                        for v in res.iter().rev() {
                            let o = v.pack(&mut builder);
                            builder.push(o);
                            os.push(o);
                        }
                        builder.start_vector::<flatbuffers::WIPOffset<#ty>>(res.len());
                        for o in os {
                            builder.push(o);
                        }
                        let o = builder.end_vector::<flatbuffers::WIPOffset<#ty>>(res.len());
                        builder.finish(o, None);
                        let data = builder.finished_data();
                        data.to_vec()
                    });
                    crate::ffi::map_result_bytes(data)
                }
            } else if result_type_str.ends_with("T") {
                c_result_type = parse_quote! {
                    *const u8
                };
                quote! {
                    let data = res.map(|res| {
                        let mut builder = flatbuffers::FlatBufferBuilder::new();
                        let ret_bytes = res.pack(&mut builder);
                        builder.finish(ret_bytes, None);
                        let data = builder.finished_data().to_vec();
                        data
                    });
                    crate::ffi::map_result_bytes(data)
                }
            } else {
                quote! {
                    crate::ffi::map_result(res)
                }
            }
        }
    };

    let ItemFn { vis, block, .. } = &input;
    let mut wrapped_fnargs = vec![];
    let mut convs = vec![];
    let mut has_coin = false;
    let mut has_network = false;
    let mut has_connection = false;
    let mut mut_connection = false;
    let mut has_client = false;
    for input in inputs.iter() {
        if let FnArg::Typed(pat) = input {
            if let Pat::Ident(param) = pat.pat.as_ref() {
                let p = pat.ty.as_ref();
                let type_name = quote! { #p }.to_string();

                let ident = &param.ident;
                let name = ident.to_string();
                match name.as_str() {
                    "coin" => {
                        has_coin = true;
                        continue;
                    }
                    "network" => {
                        has_network = true;
                        continue;
                    }
                    "connection" => {
                        has_connection = true;
                        if type_name == "& mut Connection" {
                            mut_connection = true;
                        }
                        continue;
                    }
                    "client" if is_async => {
                        has_client = true;
                        continue;
                    }
                    _ => {}
                }
                match type_name.as_str() {
                    "& str" => {
                        convs.push(quote! {
                            let #ident = unsafe { std::ffi::CStr::from_ptr(#ident).to_string_lossy() };
                            let #ident = &#ident;
                        });
                        let input: FnArg = parse_quote! {
                            #ident: *mut std::ffi::c_char
                        };
                        wrapped_fnargs.push(input);
                        continue;
                    }
                    "& [u8]" => {
                        convs.push(quote! {
                            let #ident = unsafe {
                                let ptr_data = #ident.value;
                                let len = #ident.len as usize;
                                let data = Vec::<u8>::from_raw_parts(ptr_data, len, len);
                                data
                            };
                            let #ident = &#ident[..];
                        });
                        let input: FnArg = parse_quote! {
                            #ident: crate::ffi::CParam
                        };
                        wrapped_fnargs.push(input);
                        continue;
                    }
                    _ if type_name.ends_with("T") => {
                        let tpe = Ident::new(
                            type_name
                                .strip_suffix("T")
                                .unwrap()
                                .strip_prefix("& ")
                                .unwrap(),
                            Span::call_site(),
                        );
                        convs.push(quote! {
                            let #ident = unsafe {
                                let ptr_data = #ident.value;
                                let len = #ident.len as usize;
                                let data = Vec::<u8>::from_raw_parts(ptr_data, len, len);
                                let object = flatbuffers::root::<#tpe>(&data).unwrap();
                                let obj = object.unpack();
                                tracing::debug!("{:?}", &obj);
                                obj
                            };
                            let #ident = &#ident;
                        });
                        let input: FnArg = parse_quote! {
                            #ident: crate::ffi::CParam
                        };
                        wrapped_fnargs.push(input);
                        continue;
                    }
                    _ => {}
                }
            }
        }
        wrapped_fnargs.push(input.clone());
    }

    let has_autoparam = has_coin | has_network | has_connection | mut_connection | has_client;
    let coin = if has_coin {
        quote! {
            let coin = &coin_def.clone();
        }
    } else {
        quote! {}
    };

    let network = if has_network {
        quote! {
            let network = &coin_def.network.clone();
        }
    } else {
        quote! {}
    };
    let connection = if has_connection {
        if mut_connection {
            quote! {
                let mut connection = coin_def.connection()?;
                let connection: &mut rusqlite::Connection = &mut connection;
            }
        } else {
            quote! {
                let connection = &coin_def.connection()?;
            }
        }
    } else {
        quote! {}
    };
    let client = if has_client {
        quote! {
            let mut client = coin_def.connect_lwd()?;
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

    let res = if !has_autoparam {
        quote! {
            #[no_mangle]
            pub extern "C" fn #wrapper(#(#wrapped_fnargs),*) -> crate::ffi::CResult<#c_result_type> {
                let res = || {
                    #(#convs)*
                    #ident(#(#args),*)
                };
                let res = res();
                #map_result
            }

            #vis fn #ident(#inputs) #output #block
        }
    } else if is_async {
        quote! {
            #[no_mangle]
            pub extern "C" fn #wrapper(coin: u8, #(#wrapped_fnargs),*) -> crate::ffi::CResult<#c_result_type> {
                let coin_def = {
                    let c = crate::coin::COINS[coin as usize].lock();
                    c.clone()
                };
                let runtime = coin_def.runtime.0.as_ref().unwrap();
                let res = runtime.block_on(async {
                    #coin
                    #network
                    #connection
                    #client
                    #(#convs)*
                    #ident(#(#args),*).await
                });
                #map_result
            }

            #vis async fn #ident(#inputs) #output #block
        }
    } else {
        quote! {
            #[no_mangle]
            pub extern "C" fn #wrapper(coin: u8, #(#wrapped_fnargs),*) -> crate::ffi::CResult<#c_result_type> {
                let res = || {
                    let coin_def = {
                        let c = crate::coin::COINS[coin as usize].lock();
                        c.clone()
                    };
                    #coin
                    #network
                    #connection
                    #client
                    #(#convs)*
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
