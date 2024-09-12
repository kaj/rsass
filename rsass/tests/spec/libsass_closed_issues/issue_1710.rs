//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1710.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1710")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("ul, ol {\
             \n    & & {\
             \n      display: block;\
             \n    }\
             \n  }\n"),
        "ul ul, ul ol, ol ul, ol ol {\
         \n  display: block;\
         \n}\n"
    );
}
