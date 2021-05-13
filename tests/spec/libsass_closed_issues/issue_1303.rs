//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1303.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".simple {\
             \n  a: selector-replace(\'foo.bar\', \'foo\', \'foo[baz]\');\
             \n}\n"),
        ".simple {\
         \n  a: foo.bar[baz];\
         \n}\n"
    );
}
