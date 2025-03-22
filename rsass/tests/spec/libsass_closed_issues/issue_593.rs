//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_593.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_593")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("h1:nth-of-type(#{2 + \'n + 1\'}) {\
             \n    color: red;\
             \n}\n\
             \nh1:nth-of-type(#{2 + \'n   +  1\'}) {\
             \n    color: red;\
             \n}\n"),
        "h1:nth-of-type(2n + 1) {\
         \n  color: red;\
         \n}\
         \nh1:nth-of-type(2n + 1) {\
         \n  color: red;\
         \n}\n"
    );
}
