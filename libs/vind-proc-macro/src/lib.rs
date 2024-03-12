extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input function into syntax tree
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract the function name and body
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;

    // Generate the transformed code
    let expanded = quote! {
        fn #fn_name() {
            let mut rt = vind_async::Runtime::new();
            rt.spawn(async move { #fn_body });
            rt.run();
        }
    };

    // Convert the generated code into token stream and return
    TokenStream::from(expanded)
}

