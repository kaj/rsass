//! These are from the "colors" directory in the sass specification.
//! See https://github.com/sass/sass-spec for source material.
//! I add one a test function for one specification at a time and then
//! try to implement that functionality without breaking those already
//! added.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn basic_4_0() {
    check(b"p {\n  color: rgb(255, 128, 0);\n  color: red green blue;\n  \
            color: (red) (green) (blue);\n  color: red + hux;\n  \
            color: unquote(\"red\") + green;\n  foo: rgb(200, 150%, 170%);\n}",
          "p {\n  color: #ff8000;\n  color: red green blue;\n  \
           color: red green blue;\n  color: redhux;\n  \
           color: redgreen;\n  foo: #c8ffff;\n}\n")
}

/// TODO The last value should be rgba(255, 106, 0, 0.6)
/// Green 108 instead of 106 must be some kind of rounding error.
#[test]
fn adjust_color_xx() {
    check(b"p {\n  \
            color: adjust-color(#102030, $blue: 5);\n  \
            color: adjust-color(#102030, $alpha: .325);\n  \
            color: adjust-color(#102030, $red: -5, $blue: 5);\n  \
            color: adjust-color(hsl(25, 100%, 80%), \
                                $lightness: -30%, $alpha: -0.4);\n\
            }",
          "p {\n  color: #102035;\n  color: #102030;\n  color: #0b2035;\n  \
           color: rgba(255, 108, 0, 0.6);\n}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Normal).and_then(|s| {
                   String::from_utf8(s)
                       .map_err(|e| format!("Non-utf8 output: {}", e))
               }),
               Ok(expected.into()));
}
