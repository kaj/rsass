//! Tests auto-converted from "sass-spec/spec/number-functions"
//! version 0f59164a, 2019-02-01 17:21:13 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

/// From "sass-spec/spec/number-functions/abs"
#[test]
fn abs() {
    assert_eq!(
        rsass(
            "foo {\n  foo: abs(-5);\n  foo: abs(-5px);\n  foo: abs(5);\n  foo: abs(5px);\n  foo: abs($number: 5px);\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: 5;\n  foo: 5px;\n  foo: 5;\n  foo: 5px;\n  foo: 5px;\n}\n"
    );
}

/// From "sass-spec/spec/number-functions/ceil"
#[test]
fn ceil() {
    assert_eq!(
        rsass(
            "foo {\n  foo: ceil(4.1);\n  foo: ceil(4.8px);\n  foo: ceil($number: 4.8px);\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: 5;\n  foo: 5px;\n  foo: 5px;\n}\n"
    );
}

/// From "sass-spec/spec/number-functions/floor"
#[test]
fn floor() {
    assert_eq!(
        rsass(
            "foo {\n  foo: floor(4.8);\n  foo: floor(4.8px);\n  foo: floor($number: 4.8px);\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: 4;\n  foo: 4px;\n  foo: 4px;\n}\n"
    );
}

/// From "sass-spec/spec/number-functions/max"
#[test]
fn max() {
    assert_eq!(
        rsass(
            "foo {\n  // A trailing comma forces the function to be parsed as a Sass function,\n  // rather than a CSS math function.\n  foo: max(1, 2, 3,);\n  foo: max(3, 2px, 1px,);\n  foo: max(4em,);\n  foo: max(10cm, 6in,);\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: 3;\n  foo: 3;\n  foo: 4em;\n  foo: 6in;\n}\n"
    );
}

/// From "sass-spec/spec/number-functions/min"
#[test]
fn min() {
    assert_eq!(
        rsass(
            "foo {\n  // A trailing comma forces the function to be parsed as a Sass function,\n  // rather than a CSS math function.\n  foo: min(1, 2, 3,);\n  foo: min(3px, 2px, 1,);\n  foo: min(4em,);\n  foo: min(10cm, 6in,);\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: 1;\n  foo: 1;\n  foo: 4em;\n  foo: 10cm;\n}\n"
    );
}

/// From "sass-spec/spec/number-functions/percentage"
#[test]
fn percentage() {
    assert_eq!(
        rsass(
            "foo {\n  foo: percentage(.5);\n  foo: percentage(1);\n  foo: percentage(25px / 100px);\n  foo: percentage($number: 0.5);\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: 50%;\n  foo: 100%;\n  foo: 25%;\n  foo: 50%;\n}\n"
    );
}

/// From "sass-spec/spec/number-functions/random"
#[test]
fn random() {
    assert_eq!(
        rsass(
            "foo {\n  $number: random();\n  foo: $number >= 0 and $number <= 1;\n  $number: random(1.0);\n  foo: $number >= 0 and $number <= 1;\n  foo: random(1) == 1;\n  foo: type-of(random()) == number;\n  foo: type-of(random(1)) == number;\n  foo: type-of(random(1.0)) == number;\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: true;\n  foo: true;\n  foo: true;\n  foo: true;\n  foo: true;\n  foo: true;\n}\n"
    );
}

/// From "sass-spec/spec/number-functions/round"
#[test]
fn round() {
    assert_eq!(
        rsass(
            "round {\n  positive-middle: round(1.5);\n  positive-high: round(1.51);\n  positive-low: round(1.49);\n\n  negative-middle: round(-1.5);\n  negative-high: round(-1.51);\n  negative-low: round(-1.49);\n\n  almost-middle: round(1.49999999999);\n  with-units: round(1.1px);\n  with-named: round($number: 1.1px);\n}\n"
        )
        .unwrap(),
        "round {\n  positive-middle: 2;\n  positive-high: 2;\n  positive-low: 1;\n  negative-middle: -2;\n  negative-high: -2;\n  negative-low: -1;\n  almost-middle: 1;\n  with-units: 1px;\n  with-named: 1px;\n}\n"
    );
}

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| String::from_utf8(s).map_err(|e| format!("{:?}", e)))
}
