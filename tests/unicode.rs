//! These are from the `output_styles/compressed/basic` directory in the
//! sass specification.
//! See https://github.com/sass/sass-spec for source material.
//! I add one a test function for one specification at a time and then
//! try to implement that functionality without breaking those already
//! added.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn unicode_in_normal() {
    // Blåbärsöl is a proper swedish word.  Translates to blueberry beer.
    check("a {\n  \
           content: \"Blåbärsöl\";\n\
           }",
          "@charset \"UTF-8\";\n\
           a {\n  content: \"Blåbärsöl\";\n}\n",
          OutputStyle::Normal)
}

#[test]
fn unicode_in_compressed() {
    // Blåbärsöl is a proper swedish word.  Translates to blueberry beer.
    // The charset declaration is replace with a byte order mark.
    check("a {\n  \
           content: \"Blåbärsöl\";\n\
           }",
          "\u{feff}a{content:\"Blåbärsöl\"}\n",
          OutputStyle::Compressed)
}

fn check(input: &str, expected: &str, style: OutputStyle) {
    assert_eq!(compile_scss(input.as_bytes(), style)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               expected);
}
