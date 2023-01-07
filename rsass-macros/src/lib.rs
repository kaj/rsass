//! Convert scss to css at compile-type.
//! Implemented with [rsass].
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use rsass::output::{Format, Style};
use syn::{parse_macro_input, Error, LitStr};

/// Convert a scss string to css at compile time.
///
/// The only argument is literal string of scss code.
///
/// # Examples
///
/// ````
/// use rsass_macros::scss;
/// const CSS: &str = scss!("p { em { font-style: italic; } font: serif; }");
/// assert_eq!(CSS, "p{font:serif}p em{font-style:italic}\n");
/// ````
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
        Err(err) => Error::new(Span::call_site(), format!("{err:?}"))
            .into_compile_error()
            .into(),
    }
}

/// Load and convert a scss string to css at compile time.
///
/// The only argument is a path given as a literal string.
/// The path is evaluated from the crate root of the code calling the
/// macro (the directory containing your `Cargo.toml` file).
///
/// # Examples
///
/// ````
/// use rsass_macros::include_scss;
/// const CSS: &str = include_scss!("tests/example.scss");
/// assert_eq!(CSS, "body{font-family:serif}\n");
/// ````
#[proc_macro]
pub fn include_scss(tokens: TokenStream) -> TokenStream {
    let path = parse_macro_input!(tokens as LitStr);
    let format = Format {
        style: Style::Compressed,
        precision: 10,
    };
    match rsass::compile_scss_path(path.value().as_ref(), format) {
        Ok(output) => {
            let output = core::str::from_utf8(&output).unwrap();
            LitStr::new(output, Span::call_site())
                .to_token_stream()
                .into()
        }
        Err(err) => Error::new(Span::call_site(), format!("{err:?}"))
            .into_compile_error()
            .into(),
    }
}
