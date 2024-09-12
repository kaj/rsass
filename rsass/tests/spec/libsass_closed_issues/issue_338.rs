//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_338.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_338")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n$list: (\"a\", \"b\");\n\
             \ntest {\
             \n    content: if( list.length($list) > 2, list.nth($list, 3), list.nth($list, 1) );\
             \n}\n"
        ),
        "test {\
         \n  content: \"a\";\
         \n}\n"
    );
}
