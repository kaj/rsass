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
           color: rgba(255, 106, 0, 0.6);\n}\n")
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

#[test]
fn change_color() {
    check(b"p {\n  color: change-color(#102030, $blue: 5);\n  \
            color: change-color(#102030, $alpha: .325);\n  \
            color: change-color(#102030, $red: 120, $blue: 5);\n  \
            color: change-color(hsl(25, 100%, 80%), $lightness: 40%, \
            $alpha: 0.8);\n}\n",
          "p {\n  color: #102005;\n  color: rgba(16, 32, 48, 0.325);\n  \
           color: #782005;\n  color: rgba(204, 85, 0, 0.8);\n}\n")
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
fn fade_in() {
    check(b"p {\n  color: fade-in(#f00, 0.3);\n  color: fade-in(#900, 0.8);\n  \
            color: fade-in(#000, .6);\n  color: fade-in(#fff, .2);\n  \
            color: fade-in(#999, .4);\n  color: fade-in(#000, .5);\n  \
            color: fade-in(#333, +.99);\n  // test old function name\n  \
            color: opacify(#f00, 0.3);\n  color: opacify(#900, 0.8);\n  \
            color: opacify(#000, .6);\n  color: opacify(#fff, .2);\n  \
            color: opacify(#999, .4);\n  color: opacify(#000, .5);\n  \
            color: opacify(#333, +.99);\n}",
          "p {\n  color: red;\n  color: #990000;\n  color: black;\n  \
           color: white;\n  color: #999999;\n  color: black;\n  \
           color: #333333;\n  color: red;\n  color: #990000;\n  \
           color: black;\n  color: white;\n  color: #999999;\n  \
           color: black;\n  color: #333333;\n}\n")
}

#[test]
fn fade_out() {
    check(b"p {\n  color: fade-out(#f00, 0.3);\n  \
            color: fade-out(#900, 0.8);\n  color: fade-out(#000, .6);\n  \
            color: fade-out(#fff, .2);\n  color: fade-out(#999, .4);\n  \
            color: fade-out(#000, .5);\n  color: fade-out(#333, +.99);\n  \
            // test old function name\n  \
            color: transparentize(#f00, 0.3);\n  \
            color: transparentize(#900, 0.8);\n  \
            color: transparentize(#000, .6);\n  \
            color: transparentize(#fff, .2);\n  \
            color: transparentize(#999, .4);\n  \
            color: transparentize(#000, .5);\n  \
            color: transparentize(#333, +.99);\n}",
          "p {\n  color: rgba(255, 0, 0, 0.7);\n  \
           color: rgba(153, 0, 0, 0.2);\n  color: rgba(0, 0, 0, 0.4);\n  \
           color: rgba(255, 255, 255, 0.8);\n  \
           color: rgba(153, 153, 153, 0.6);\n  color: rgba(0, 0, 0, 0.5);\n  \
           color: rgba(51, 51, 51, 0.01);\n  color: rgba(255, 0, 0, 0.7);\n  \
           color: rgba(153, 0, 0, 0.2);\n  color: rgba(0, 0, 0, 0.4);\n  \
           color: rgba(255, 255, 255, 0.8);\n  \
           color: rgba(153, 153, 153, 0.6);\n  color: rgba(0, 0, 0, 0.5);\n  \
           color: rgba(51, 51, 51, 0.01);\n}\n")
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
