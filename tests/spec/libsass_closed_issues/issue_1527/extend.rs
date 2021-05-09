//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1527/extend.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "foo {\
             \n  @extend &;\
             \n}\n"
        ),
        "Error: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n2 |   @extend &;\
         \n  |           ^\
         \n  \'\
         \n  input.scss 2:11  root stylesheet",
    );
}
