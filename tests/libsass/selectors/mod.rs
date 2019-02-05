//! Tests auto-converted from "sass-spec/spec/libsass/selectors"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/libsass/selectors/access"
#[test]
#[ignore] // failing
fn access() {
    assert_eq!(
        rsass(
            "@mixin selector-access {\n  mixin-sees: &;\n}\n\n@function function-access() {\n  @return &;\n}\n\n.foo {\n  @include selector-access;\n  function-sees: function-access();\n}\n\n.bar a {\n  @include selector-access;\n  function-sees: function-access();\n}\n\n.bar,\n.baz {\n  @include selector-access;\n  function-sees: function-access();\n}\n\n.qux {\n  &.waldo {\n    .where & {\n      .final {\n        @include selector-access;\n        function-sees: function-access();\n      }\n    }\n  }\n}"
        )
        .unwrap(),
        ".foo {\n  mixin-sees: .foo;\n  function-sees: .foo;\n}\n.bar a {\n  mixin-sees: .bar a;\n  function-sees: .bar a;\n}\n.bar,\n.baz {\n  mixin-sees: .bar, .baz;\n  function-sees: .bar, .baz;\n}\n.where .qux.waldo .final {\n  mixin-sees: .where .qux.waldo .final;\n  function-sees: .where .qux.waldo .final;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/selectors/function-argument"
#[test]
fn function_argument() {
    assert_eq!(
        rsass(
            "$Selectors: ();\n\n//////////////////////////////\n// Add selectors of various depths and makeups\n//////////////////////////////\n.foo {\n  $Selectors: append($Selectors, &) !global;\n}\n\n.bar a {\n  $Selectors: append($Selectors, &) !global;\n}\n\n.bar,\n.baz {\n  $Selectors: append($Selectors, &) !global;\n}\n\n.qux {\n  &.waldo {\n    .where & {\n      .final {\n        $Selectors: append($Selectors, &) !global;\n      }\n    }\n  }\n}\n\n//////////////////////////////\n// Display Results\n//////////////////////////////\n.result {\n  length: length($Selectors);\n  content: $Selectors;\n  @for $i from 1 through length($Selectors) {\n    index: $i;\n    length: length(nth($Selectors, $i));\n    content: nth($Selectors, $i);\n  }\n}"
        )
        .unwrap(),
        ".result {\n  length: 4;\n  content: .foo .bar a .bar, .baz .where .qux.waldo .final;\n  index: 1;\n  length: 1;\n  content: .foo;\n  index: 2;\n  length: 1;\n  content: .bar a;\n  index: 3;\n  length: 2;\n  content: .bar, .baz;\n  index: 4;\n  length: 1;\n  content: .where .qux.waldo .final;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/selectors/interpolation"
#[test]
#[ignore] // failing
fn interpolation() {
    assert_eq!(
        rsass(
            ".foo {\n  content: #{&};\n}\n\n.bar a {\n  content: #{&};\n}\n\n.bar,\n.baz {\n  content: #{&};\n}\n\n.qux {\n  &.waldo {\n    .where & {\n      .final {\n        content: #{&};\n      }\n    }\n  }\n}"
        )
        .unwrap(),
        ".foo {\n  content: .foo;\n}\n.bar a {\n  content: .bar a;\n}\n.bar,\n.baz {\n  content: .bar, .baz;\n}\n.where .qux.waldo .final {\n  content: .where .qux.waldo .final;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/selectors/mixin-argument"
#[test]
#[ignore] // failing
fn mixin_argument() {
    assert_eq!(
        rsass(
            "@mixin selector-info($selector) {\n  type: type-of($selector);\n  length: length($selector);\n  content: $selector;\n  @for $i from 1 through length($selector) {\n    index: $i;\n    length: length(nth($selector, $i));\n    type: type-of(nth($selector, $i));\n    content: nth($selector, $i);\n  }\n}\n\n.foo {\n  @include selector-info(&);\n}\n\n.bar a {\n  @include selector-info(&);\n}\n\n.bar,\n.baz {\n  @include selector-info(&);\n}\n\n.qux {\n  &.waldo {\n    .where & {\n      .final {\n        @include selector-info(&);\n      }\n    }\n  }\n}"
        )
        .unwrap(),
        ".foo {\n  type: list;\n  length: 1;\n  content: .foo;\n  index: 1;\n  length: 1;\n  type: list;\n  content: .foo;\n}\n.bar a {\n  type: list;\n  length: 1;\n  content: .bar a;\n  index: 1;\n  length: 2;\n  type: list;\n  content: .bar a;\n}\n.bar,\n.baz {\n  type: list;\n  length: 2;\n  content: .bar, .baz;\n  index: 1;\n  length: 1;\n  type: list;\n  content: .bar;\n  index: 2;\n  length: 1;\n  type: list;\n  content: .baz;\n}\n.where .qux.waldo .final {\n  type: list;\n  length: 1;\n  content: .where .qux.waldo .final;\n  index: 1;\n  length: 3;\n  type: list;\n  content: .where .qux.waldo .final;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/selectors/simple"
#[test]
#[ignore] // failing
fn simple() {
    assert_eq!(
        rsass(
            "div {\n  span, p, span {\n    color: red;\n  }\n  a.foo.bar.foo {\n    color: green;\n  }\n  &:nth(-3) {\n    color: blue;\n  }\n}\n\n@-webkit-keyframes {\n  from {\n    left: 0px;\n    10% {\n      whatever: hoo;\n    }\n  }\n  to {\n    left: 200px;\n  }\n}\n\ndiv {\n  @whatever {\n    blah: blah;\n    stuff {\n      blah: bloh;\n    }\n  }\n}\n\n\na, b {\n  color: red;\n  c, d {\n    height: 10px;\n    e, f {\n      width: 12px;\n    }\n  }\n}\n"
        )
        .unwrap(),
        "div span, div p, div span {\n  color: red;\n}\ndiv a.foo.bar.foo {\n  color: green;\n}\ndiv:nth(-3) {\n  color: blue;\n}\n@-webkit-keyframes {\n  from {\n    left: 0px;\n    10% {\n      whatever: hoo;\n    }\n  }\n  to {\n    left: 200px;\n  }\n}\n@whatever {\n  div {\n    blah: blah;\n  }\n  div stuff {\n    blah: bloh;\n  }\n}\na, b {\n  color: red;\n}\na c, a d, b c, b d {\n  height: 10px;\n}\na c e, a c f, a d e, a d f, b c e, b c f, b d e, b d f {\n  width: 12px;\n}\n"
    );
}

mod variables;
