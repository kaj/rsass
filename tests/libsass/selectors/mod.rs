//! Tests auto-converted from "sass-spec/spec/libsass/selectors"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "access", not expected to work yet.

/// From "sass-spec/spec/libsass/selectors/function-argument"
#[test]
fn function_argument() {
    assert_eq!(
        rsass(
            "$Selectors: ();\n//////////////////////////////\n// Add selectors of various depths and makeups\n//////////////////////////////\n.foo {\n  $Selectors: append($Selectors, &) !global;\n}\n.bar a {\n  $Selectors: append($Selectors, &) !global;\n}\n.bar,\n.baz {\n  $Selectors: append($Selectors, &) !global;\n}\n.qux {\n  &.waldo {\n    .where & {\n      .final {\n        $Selectors: append($Selectors, &) !global;\n      }\n    }\n  }\n}\n//////////////////////////////\n// Display Results\n//////////////////////////////\n.result {\n  length: length($Selectors);\n  content: $Selectors;\n  @for $i from 1 through length($Selectors) {\n    index: $i;\n    length: length(nth($Selectors, $i));\n    content: nth($Selectors, $i);\n  }\n}"
        )
        .unwrap(),
        ".result {\n  length: 4;\n  content: .foo .bar a .bar, .baz .where .qux.waldo .final;\n  index: 1;\n  length: 1;\n  content: .foo;\n  index: 2;\n  length: 1;\n  content: .bar a;\n  index: 3;\n  length: 2;\n  content: .bar, .baz;\n  index: 4;\n  length: 1;\n  content: .where .qux.waldo .final;\n}\n"
    );
}

// Ignoring "interpolation", not expected to work yet.

// Ignoring "mixin-argument", not expected to work yet.

// Ignoring "simple", not expected to work yet.

mod variables;
