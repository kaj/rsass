//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1527/extend.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
             \n  @extend &;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n2 |   @extend &;\
         \n  |           ^\
         \n  \'\
         \n  input.scss 2:11  root stylesheet\
         \n",
    );
}
