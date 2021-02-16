//! Tests from spec/scss
use rsass::compile_scss;

/// My own addition
#[test]
fn important_compact_input() {
    check(
        "div {z-index: 40!important}",
        "div {\n  z-index: 40 !important;\n}\n",
    )
}

/// My own addition
#[test]
fn interpolation_sq() {
    check(
        "$bar : '#foo';\n\n\
         ul li#{$bar} a span.label { foo: bar; }\n",
        "ul li#foo a span.label {\n  foo: bar;\n}\n",
    )
}

#[test]
fn comparable() {
    // TODO One line, involving a fictional unit, removed here
    check(
        ".color-functions {
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
",
    )
}

#[test]
fn media2() {
    check(
        "$foo: 3;\n$bar: 4;\n\n\
         @media only screen and (max-width: $foo) and (min-width: $bar) {\n  \
         /* hey */\n  \
         a {color: red}\n}",
        "@media only screen and (max-width: 3) and (min-width: 4) {\n  \
         /* hey */\n  \
         a {\n    color: red;\n  }\n}\n",
    )
}

#[test]
fn additional_selectors() {
    check("
    [class$=\"--foo\"], [class~=\"--foo\"], , [class^=\"--foo\"] {\
    \n  x: y;\
    \n}\
    \n", "[class$=\"--foo\"], [class~=\"--foo\"], [class^=\"--foo\"] {\n  x: y;\n}\n")
}

fn check(input: &str, expected: &str) {
    assert_eq!(
        String::from_utf8(
            compile_scss(input.as_ref(), Default::default()).unwrap()
        )
        .unwrap(),
        expected
    );
}
