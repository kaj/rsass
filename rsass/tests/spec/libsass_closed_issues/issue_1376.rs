//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1376.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1376")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".div{\
             \n  $foo: 1, null, 2, null, 3;\n\
             \n  content: \"#{$foo}\";\
             \n}"),
        ".div {\
         \n  content: \"1, 2, 3\";\
         \n}\n"
    );
}
