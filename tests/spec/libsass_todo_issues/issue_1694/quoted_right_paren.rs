//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1694/quoted-right-paren.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\
             \n  filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20\\);\
             \n}\
             \n"
        ).unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n2 |   filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20\\);\
         \n  |                                                               ^\
         \n  \'\
         \n  input.scss 2:63  root stylesheet\
         \n",
    );
}
