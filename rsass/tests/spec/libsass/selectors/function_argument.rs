//! Tests auto-converted from "sass-spec/spec/libsass/selectors/function-argument.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function-argument")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n$Selectors: ();\n\
             \n//////////////////////////////\
             \n// Add selectors of various depths and makeups\
             \n//////////////////////////////\
             \n.foo {\
             \n  $Selectors: list.append($Selectors, &) !global;\
             \n}\n\
             \n.bar a {\
             \n  $Selectors: list.append($Selectors, &) !global;\
             \n}\n\
             \n.bar,\
             \n.baz {\
             \n  $Selectors: list.append($Selectors, &) !global;\
             \n}\n\
             \n.qux {\
             \n  &.waldo {\
             \n    .where & {\
             \n      .final {\
             \n        $Selectors: list.append($Selectors, &) !global;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n//////////////////////////////\
             \n// Display Results\
             \n//////////////////////////////\
             \n.result {\
             \n  length: list.length($Selectors);\
             \n  content: $Selectors;\
             \n  @for $i from 1 through list.length($Selectors) {\
             \n    index: $i;\
             \n    length: list.length(list.nth($Selectors, $i));\
             \n    content: list.nth($Selectors, $i);\
             \n  }\
             \n}"),
        ".result {\
         \n  length: 4;\
         \n  content: .foo .bar a .bar, .baz .where .qux.waldo .final;\
         \n  index: 1;\
         \n  length: 1;\
         \n  content: .foo;\
         \n  index: 2;\
         \n  length: 1;\
         \n  content: .bar a;\
         \n  index: 3;\
         \n  length: 2;\
         \n  content: .bar, .baz;\
         \n  index: 4;\
         \n  length: 1;\
         \n  content: .where .qux.waldo .final;\
         \n}\n"
    );
}
