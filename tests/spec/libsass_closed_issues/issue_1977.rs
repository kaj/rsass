//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1977.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1977")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("body#some-\\(selector\\) {\
             \ncolor: red;\
             \n}\n\
             \n#äöü  {\
             \n  color: reds;\
             \n}"),
        "@charset \"UTF-8\";\
         \nbody#some-\\(selector\\) {\
         \n  color: red;\
         \n}\
         \n#äöü {\
         \n  color: reds;\
         \n}\n"
    );
}
