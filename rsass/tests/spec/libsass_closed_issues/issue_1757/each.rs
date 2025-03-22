//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1757/each.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("each")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n.test .nest {\
             \n  length: list.length(&);\
             \n  @each $list in & {\
             \n    list: $list;\
             \n    length: list.length($list);\
             \n  }\
             \n}\n\
             \n.test, .other {\
             \n  length: list.length(&);\
             \n  @each $list in & {\
             \n    list: $list;\
             \n    length: list.length($list);\
             \n  }\
             \n}\n"),
        ".test .nest {\
         \n  length: 1;\
         \n  list: .test .nest;\
         \n  length: 2;\
         \n}\
         \n.test, .other {\
         \n  length: 2;\
         \n  list: .test;\
         \n  length: 1;\
         \n  list: .other;\
         \n  length: 1;\
         \n}\n"
    );
}
