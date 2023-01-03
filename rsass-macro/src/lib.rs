extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{Error, LitStr, parse_macro_input};
use rsass::output::{Format, Style};

#[proc_macro]
pub fn scss(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as LitStr);
    let format = Format {
        style: Style::Compressed,
        precision: 10,
    };
    match rsass::compile_scss(input.value().as_bytes(), format) {
        Ok(output) => {
            let output = core::str::from_utf8(&output).unwrap();
            LitStr::new(output, Span::call_site())
                .to_token_stream()
                .into()
        }
        Err(err) => {
            let msg = format!("{err:?}");
            Error::new(Span::call_site(), msg)
                .into_compile_error()
                .into()
        }
    }
}