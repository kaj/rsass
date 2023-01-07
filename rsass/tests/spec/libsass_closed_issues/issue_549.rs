//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_549.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_549")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$value: 10;\n\
             \nfoo {\
             \n  filter: foo(opacity=$value*100);\
             \n}\n"),
        "foo {\
         \n  filter: foo(opacity=1000);\
         \n}\n"
    );
}
