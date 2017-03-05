//! Tests from spec/scss
extern crate rsass;
use rsass::{OutputStyle, compile_scss};


#[test]
fn important() {
    check(b"div {\n  color: red ! important;\n  width: 5px ! important;\n}",
          "div {\n  color: red !important;\n  width: 5px !important;\n}\n")
}

/// My own addition
#[test]
fn important_compact_input() {
    check(b"div {z-index: 40!important}",
          "div {\n  z-index: 40 !important;\n}\n")
}

#[test]
fn comparable() {
    // TODO On line, involving a fictional unit, removed here
    check(b".color-functions {
  $color: red;
  hue: hue($color);
  hue-type: type-of(hue($color));
  hue-unit: unit(hue($color));
  hue-comparable: comparable(hue($color), hue($color));
  x: comparable(10px, 10px);
  x: comparable(10, 10);
  y: type-of(10px);
  y: type-of(10deg);
  z: lightness(red);
  z: type-of(lightness(red));
  z: type-of(50%);
  z: comparable(lightness(red), 50%);
}",
          ".color-functions {
  hue: 0deg;
  hue-type: number;
  hue-unit: \"deg\";
  hue-comparable: true;
  x: true;
  x: true;
  y: number;
  y: number;
  z: 50%;
  z: number;
  z: number;
  z: true;
}
")
}


fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Normal).and_then(|s| {
                   String::from_utf8(s)
                       .map_err(|e| format!("Non-utf8 output: {}", e))
               }),
               Ok(expected.into()));
}
