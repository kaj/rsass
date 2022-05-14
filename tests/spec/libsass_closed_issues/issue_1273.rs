//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1273.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1273")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("test {\
             \n  src: url(test.eot#{if(true, \'?#{42}\', \'\')});\
             \n}"),
        "test {\
         \n  src: url(test.eot?42);\
         \n}\n"
    );
}
