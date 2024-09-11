extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse2, parse_macro_input, FnArg, GenericArgument, Ident, ItemFn, Pat, PathArguments, ReturnType, Signature, Type
};

#[proc_macro_attribute]
pub fn c_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;

    let Signature {
        ident,
        inputs,
        output,
        ..
    } = sig;

    let ReturnType::Type(_, retype) = output else {
        panic!()
    };
    let Type::Path(type_path) = retype.as_ref() else {
        panic!()
    };
    let path = &type_path.path;
    assert_eq!(path.segments[0].ident.to_string(), "Result");
    let PathArguments::AngleBracketed(type_arg) = &path.segments[0].arguments else {
        panic!()
    };
    let GenericArgument::Type(result_type) = &type_arg.args[0] else {
        panic!()
    };
    let Type::Path(result_type) = result_type else {
        panic!()
    };
    let result_type = result_type.path.get_ident().unwrap();
    let map_result = match result_type.to_string().as_str() {
        "String" => quote! {
            map_result_string(res)
        },
        "VecBytes" => quote! {
            map_result_bytes(res)
        },
        _ => quote! {
            map_result(res)
        },
    };

    let ItemFn { vis, block, .. } = &input;
    let mut wrapped_fnargs = vec![];
    let mut str_convs = vec![];
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
                    "client" => {
                        has_client = true;
                        continue;
                    }
                    _ => {}
                }
                let p = pat.ty.as_ref();
                let s = quote! { #p }.to_string();
                if s == "& str" {
                    str_convs.push(quote! {
                        let #ident = unsafe { CStr::from_ptr(#ident).to_string_lossy() };
                        let #ident = &#ident;
                    });
                    let input = quote! {
                        #ident: *mut c_char
                    };
                    let input = parse2::<FnArg>(input).unwrap();
                    wrapped_fnargs.push(input);
                    continue;
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
            let connection = &coin.connection();
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
                panic!();
            };
            let syn::Pat::Ident(pat_ident) = &*pat_type.pat else {
                panic!();
            };
            &pat_ident.ident
        })
        .collect::<Vec<_>>();

    let wrapper = Ident::new(&format!("c_{}", ident), ident.span());

    let res = quote! {
        #[no_mangle]
        #[tokio::main]
        pub async extern "C" fn #wrapper(coin: u8, #(#wrapped_fnargs),*) -> CResult<#result_type> {
            let res = async {
                let coin = COINS[coin as usize].lock();
                #network
                #connection
                #client
                #(#str_convs),*
                #ident(#(#args),*).await
            };
            let res = res.await;
            #map_result
        }

        #vis async fn #ident(#inputs) #output #block
    };

    res.into()
}
