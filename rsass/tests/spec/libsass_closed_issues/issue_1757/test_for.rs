//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1757/for.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("for")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".test .nest {\
             \n  length: length(&);\
             \n  @for $i from 1 through length(&) {\
             \n    $list: nth(&, $i);\
             \n    list: $list;\
             \n    length: length($list);\
             \n  }\
             \n}\n\
             \n.test, .other {\
             \n  length: length(&);\
             \n  @for $i from 1 through length(&) {\
             \n    $list: nth(&, $i);\
             \n    list: $list;\
             \n    length: length($list);\
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
