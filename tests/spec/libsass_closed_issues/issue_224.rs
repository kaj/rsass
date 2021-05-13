//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_224.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$list: (\"a\", \"b\", \"c\");\n\
             \ntest {\
             \n    content: nth($list, -1);\
             \n    content: nth($list, -2);\
             \n    content: nth($list, -3);\
             \n    content: nth($list, -1) == nth($list, length($list));\
             \n}\n"),
        "test {\
         \n  content: \"c\";\
         \n  content: \"b\";\
         \n  content: \"a\";\
         \n  content: true;\
         \n}\n"
    );
}
