//! Tests auto-converted from "sass-spec/spec/libsass/selectors/mixin-argument.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin-argument")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@mixin selector-info($selector) {\
             \n  type: meta.type-of($selector);\
             \n  length: list.length($selector);\
             \n  content: $selector;\
             \n  @for $i from 1 through list.length($selector) {\
             \n    index: $i;\
             \n    length: list.length(list.nth($selector, $i));\
             \n    type: meta.type-of(list.nth($selector, $i));\
             \n    content: list.nth($selector, $i);\
             \n  }\
             \n}\n\
             \n.foo {\
             \n  @include selector-info(&);\
             \n}\n\
             \n.bar a {\
             \n  @include selector-info(&);\
             \n}\n\
             \n.bar,\
             \n.baz {\
             \n  @include selector-info(&);\
             \n}\n\
             \n.qux {\
             \n  &.waldo {\
             \n    .where & {\
             \n      .final {\
             \n        @include selector-info(&);\
             \n      }\
             \n    }\
             \n  }\
             \n}"),
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
         \n}\n"
    );
}
