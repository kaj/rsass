//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1931.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1931")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$var: \'http://test.com\';\
             \nbody {\
             \n  background-image: url( #{$var});\
             \n}"),
        "body {\
         \n  background-image: url(http://test.com);\
         \n}\n"
    );
}
