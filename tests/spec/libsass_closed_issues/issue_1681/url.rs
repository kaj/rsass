//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1681/url.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@function url() {\
             \n  @return null;\
             \n}\n"
        ),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function url() {\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
