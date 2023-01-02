extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{LitStr, parse_macro_input};
use rsass::output::{Format, Style};

#[proc_macro]
pub fn scss(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as LitStr);

    LitStr::new(core::str::from_utf8( &rsass::compile_scss( input.value().as_bytes(), Format {
        style: Style::Compressed,
        precision: 10,
    }).unwrap()).unwrap(), Span::call_site()).to_token_stream().into()
}