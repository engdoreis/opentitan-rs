// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, ItemFn};

#[proc_macro_attribute]
pub fn test_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as ItemFn);

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let args = f.sig.inputs;
    let stmts = f.block.stmts;

    quote!(
        #[allow(non_snake_case)]
        #[export_name = "test_entry"]
        #(#attrs)*
        pub #unsafety fn _test_utils__test_main(#args) -> bool{
            #(#stmts)*
        }
    )
    .into()
}

#[proc_macro_derive(PlicVectorTable)]
pub fn derive_plic_vector_table(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let Data::Enum(data_enum) = ast.data else {
        panic!("PlicVectorTable can only be derived on enums");
    };

    let variant_names: Vec<_> = data_enum.variants.iter().map(|v| &v.ident).collect();
    let variants = variant_names.len();

    // Generate the weak functions and the array.
    let expanded = quote! {
        #[no_mangle]
        #[inline(never)]
        pub fn DefaultPlicHandler(name: &str) {
            panic!("Unhandled PLIC Interrupt {}!", name);
        }
    #(
        #[no_mangle]
        #[linkage = "weak"]
        pub extern "C" fn #variant_names() {
            DefaultPlicHandler(stringify!(#variant_names));
        }
    )*

        // Generate array of plic handlers for every interrupt source in the enum
        static PLIC_VECTOR_TABLE: [unsafe extern "C" fn(); #variants] = [
            #( #variant_names, )*
        ];
    };
    TokenStream::from(expanded)
}
