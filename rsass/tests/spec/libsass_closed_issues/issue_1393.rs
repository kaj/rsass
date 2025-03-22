//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1393.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1393")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  back#{ground}: {\
             \n    imag#{e}: url(foo.png);\
             \n    pos#{it}ion: 50%;\
             \n  }\
             \n}\n"),
        "div {\
         \n  background-image: url(foo.png);\
         \n  background-position: 50%;\
         \n}\n"
    );
}
