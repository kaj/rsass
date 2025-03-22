//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1694/quoted-right-paren.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("quoted-right-paren")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "test {\
             \n  filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20\\);\
             \n}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n2 |   filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20\\);\
         \n  |                                                               ^\
         \n  \'\
         \n  input.scss 2:63  root stylesheet",
    );
}
