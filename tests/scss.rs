//! Tests from spec/scss
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn alpha() {
    check(b"$x: rgb(0, 255, 255);\n\n\
            div {\n  color: rgb(255, $blue: 0, $green: 255);\n  \
            background: rgb(123, 45, 6);\n//  \
            flah: rgba(0, 0, 0, 1) + #111;\n  \
            grah: rgba(#f0e, $alpha: .5);\n//  blah: rgba(1,2,3,.6);\n\n  \
            floo: $x;\n//  bloo: rgba($x, 0.7);\n  groo: $x;\n\n  \
            $x: rgb(123, 45, 6);\n\n  \
            hoo: red($x);\n\n  moo: green($x);\n  poo: blue($x);\n\n\
            //  goo: mix(rgba(255, 0, 0, 0.5), #00f);\n\n\
            boo: invert(#123456);\n}\n",
          "div {\n  color: yellow;\n  background: #7b2d06;\n  \
           grah: rgba(255, 0, 238, 0.5);\n  floo: cyan;\n  groo: cyan;\n  \
           hoo: 123;\n  moo: 45;\n  poo: 6;\n  boo: #edcba9;\n}\n")
}

#[test]
fn append() {
    check(b"div {\n  \
            $l: append(a b, c d);\n  foo: nth($l, 3);\n  bar: type-of($l);\n}",
          "div {\n  foo: c d;\n  bar: list;\n}\n")
}

#[test]
fn arglist() {
    check(b"@mixin foo($x, $y, $zs...) {\n  \
            foo-x: $x;\n  foo-y: $y;\n  foo-zs: $zs;\n}\n\n\
            div {\n  @include foo(a, b, c, d, e);\n}",
          "div {\n  foo-x: a;\n  foo-y: b;\n  foo-zs: c, d, e;\n}\n")
}

#[test]
fn backrefs_in_selector_groups() {
    check(b"a {\n  &:c, & d {\n    hey: ho;\n  }\n}\n\n\
            a b {\n  &:c, & d {\n    hey: ho;\n  }\n}\n",
          "a:c, a d {\n  hey: ho;\n}\n\n\
           a b:c, a b d {\n  hey: ho;\n}\n")
}

#[test]
fn backslash() {
    check(b"div, span {\n  color: red;\n  \\ foo {\n    color: blue;\n  }\n}",
          "div, span {\n  color: red;\n}\n\
           div \\ foo, span \\ foo {\n  color: blue;\n}\n")
}

#[test]
fn basic_function() {
    check(b"@function foo() {\n  @return 1 + 2;\n}\n\n\
            bar {\n  a: foo();\n}\n",
          "bar {\n  a: 3;\n}\n")
}

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

#[test]
fn index() {
    check(b"div {\n  foo: index(hello goodbye futz, goodbye);\n  \
            bar: index(hello goodbye futz, badbye);\n  \
            baz: index((hello world) (my name) (is aaron), is aaron);\n}",
          "div {\n  foo: 2;\n  baz: 3;\n}\n")
}

#[test]
fn star_plus_and_parent() {
    check(b"foo {*+html & {a: b}}\n",
          "* + html foo {\n  a: b;\n}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Normal).and_then(|s| {
                   String::from_utf8(s)
                       .map_err(|e| format!("Non-utf8 output: {}", e))
               }),
               Ok(expected.into()));
}
