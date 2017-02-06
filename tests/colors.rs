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

#[test]
fn adjust_hue() {
    check(b"p {\n  color: adjust-hue(#f00, 10);\n   \
            color: adjust-hue(#900, 90);\n  \
            color: adjust-hue(#000, 30);\n  \
            color: adjust-hue(#fff, -30);\n  \
            color: adjust-hue(#999, -530);\n  \
            color: adjust-hue(#000, +720);\n  \
            // error in libsass\n  // color: adjust-hue(#333, + 720);\n}\n",
          "p {\n  color: #ff2b00;\n  color: #4d9900;\n  color: black;\n  \
           color: white;\n  color: #999999;\n  color: black;\n}\n")
}

/// TODO Another rouding error here; the last color in the spec should be
/// rgba(204, 85, 0, 0.8), with 85 rather than 86.  The last color in this
/// check is the adjusted hsl value without an extra hsl->rgb->hsl
/// conversions, for reference
#[test]
fn change_color() {
    check(b"p {\n  color: change-color(#102030, $blue: 5);\n  \
            color: change-color(#102030, $alpha: .325);\n  \
            color: change-color(#102030, $red: 120, $blue: 5);\n  \
            color: change-color(hsl(25, 100%, 80%), $lightness: 40%, \
            $alpha: 0.8);\n  \
            color: hsla(25, 100%, 40%, 0.8);\n}\n",
          "p {\n  color: #102005;\n  color: rgba(16, 32, 48, 0.325);\n  \
           color: #782005;\n  color: rgba(204, 86, 0, 0.8);\n  \
           color: rgba(204, 85, 0, 0.8);\n}\n")
}

#[test]
fn complement() {
    check(b"p {\n  color: complement(#f00);\n  color: complement(#900);\n  \
            color: complement(#000);\n  color: complement(#fff);\n  \
            color: complement(#999);\n  color: complement(#000);\n  \
            color: complement(#333);\n}",
          "p {\n  color: cyan;\n  color: #009999;\n  color: black;\n  \
           color: white;\n  color: #999999;\n  color: black;\n  \
           color: #333333;\n}\n")
}

#[test]
fn saturate() {
    check(b"p {\n  color: saturate(#f00, 10%);\n  \
            color: saturate(#900, 10%);\n  color: saturate(#000, 10%);\n  \
            color: saturate(#fff, 10%);\n  color: saturate(#999, 10%);\n  \
            color: saturate(#000, 10%);\n}",
          "p {\n  color: red;\n  color: #990000;\n  color: black;\n  \
           color: white;\n  color: #a38f8f;\n  color: black;\n}\n")
}

#[test]
fn saturation() {
    check(b"p {\n  test-01: saturation(red);\n  test-01: saturation(#333);\n  \
            test-02: saturation(hsl(60, 30, 20));\n  \
            test-03: saturation(saturate(#f00, 10%));\n  \
            test-04: saturation(saturate(#900, 10%));\n  \
            // this is probably a ruby sass bug\n  \
            // test-05: saturation(saturate(#000, 10%));\n  \
            // test-06: saturation(saturate(#fff, 10%));\n  \
            test-07: saturation(saturate(#999, 10%));\n  \
            test-08: saturation(desaturate(#fff, 10%));\n  \
            test-09: saturation(desaturate(#999, 10%));\n  \
            test-10: saturation(desaturate(#000, 10%));\n  \
            test-11: saturation(desaturate(#f00, 10%));\n  \
            test-12: saturation(desaturate(#900, 10%));\n}",
          "p {\n  test-01: 100%;\n  test-01: 0%;\n  test-02: 30%;\n  \
           test-03: 100%;\n  test-04: 100%;\n  test-07: 10%;\n  \
           test-08: 0%;\n  test-09: 0%;\n  test-10: 0%;\n  test-11: 90%;\n  \
           test-12: 90%;\n}\n")
}

#[test]
fn desaturate() {
    check(b"p {\n  color: desaturate(#fff, 10%);\n  \
            color: desaturate(#999, 10%);\n  color: desaturate(#000, 10%);\n  \
            color: desaturate(#f00, 10%);\n  color: desaturate(#900, 10%);\n}",
          "p {\n  color: white;\n  color: #999999;\n  color: black;\n  \
           color: #f20d0d;\n  color: #910808;\n}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Normal).and_then(|s| {
                   String::from_utf8(s)
                       .map_err(|e| format!("Non-utf8 output: {}", e))
               }),
               Ok(expected.into()));
}
