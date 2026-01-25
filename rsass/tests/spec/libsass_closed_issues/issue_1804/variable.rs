//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1804/variable.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("variable")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("$foo: 2px;\
             \n$bar: 5in;\n\
             \nfoo {\
             \n  bar: #{($foo*$bar)};\
             \n}\n"),
        "foo {\
         \n  bar: calc(10px * 1in);\
         \n}\n"
    );
}
