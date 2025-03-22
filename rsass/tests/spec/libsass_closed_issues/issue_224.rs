//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_224.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_224")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n$list: (\"a\", \"b\", \"c\");\n\
             \ntest {\
             \n    content: list.nth($list, -1);\
             \n    content: list.nth($list, -2);\
             \n    content: list.nth($list, -3);\
             \n    content: list.nth($list, -1) == list.nth($list, list.length($list));\
             \n}\n"
        ),
        "test {\
         \n  content: \"c\";\
         \n  content: \"b\";\
         \n  content: \"a\";\
         \n  content: true;\
         \n}\n"
    );
}
