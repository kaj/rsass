//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_338.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "$list: (\"a\", \"b\");\n\
             \ntest {\
             \n    content: if( length($list) > 2, nth($list, 3), nth($list, 1) );\
             \n}\n"
        ),
        "test {\
         \n  content: \"a\";\
         \n}\n"
    );
}
