//! These are from the `css` directory in the sass specification.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn multi_star_comments() {
    // Note, actual expected has single newlines, but the sass-spec
    // test runner succeeds my implementation.
    check(b"a /***/ b {x: y}\n\
            a /****/ b {x: y}\n\
            a /* **/ b {x: y}\n\
            a /** */ b {x: y}\n",
          "a b {\n  x: y;\n}\n\n\
           a b {\n  x: y;\n}\n\n\
           a b {\n  x: y;\n}\n\n\
           a b {\n  x: y;\n}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Normal)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               expected);
}
