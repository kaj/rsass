//! Tests auto-converted from "sass-spec/spec/libsass/selectors/mixin-argument.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin selector-info($selector) {\
            \n  type: type-of($selector);\
            \n  length: length($selector);\
            \n  content: $selector;\
            \n  @for $i from 1 through length($selector) {\
            \n    index: $i;\
            \n    length: length(nth($selector, $i));\
            \n    type: type-of(nth($selector, $i));\
            \n    content: nth($selector, $i);\
            \n  }\
            \n}\
            \n\
            \n.foo {\
            \n  @include selector-info(&);\
            \n}\
            \n\
            \n.bar a {\
            \n  @include selector-info(&);\
            \n}\
            \n\
            \n.bar,\
            \n.baz {\
            \n  @include selector-info(&);\
            \n}\
            \n\
            \n.qux {\
            \n  &.waldo {\
            \n    .where & {\
            \n      .final {\
            \n        @include selector-info(&);\
            \n      }\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  type: list;\
        \n  length: 1;\
        \n  content: .foo;\
        \n  index: 1;\
        \n  length: 1;\
        \n  type: list;\
        \n  content: .foo;\
        \n}\
        \n.bar a {\
        \n  type: list;\
        \n  length: 1;\
        \n  content: .bar a;\
        \n  index: 1;\
        \n  length: 2;\
        \n  type: list;\
        \n  content: .bar a;\
        \n}\
        \n.bar,\
        \n.baz {\
        \n  type: list;\
        \n  length: 2;\
        \n  content: .bar, .baz;\
        \n  index: 1;\
        \n  length: 1;\
        \n  type: list;\
        \n  content: .bar;\
        \n  index: 2;\
        \n  length: 1;\
        \n  type: list;\
        \n  content: .baz;\
        \n}\
        \n.where .qux.waldo .final {\
        \n  type: list;\
        \n  length: 1;\
        \n  content: .where .qux.waldo .final;\
        \n  index: 1;\
        \n  length: 3;\
        \n  type: list;\
        \n  content: .where .qux.waldo .final;\
        \n}\
        \n"
    );
}
