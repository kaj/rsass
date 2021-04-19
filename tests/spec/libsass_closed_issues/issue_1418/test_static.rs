//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1418/static.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
             \n    color: missing($a: b);\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Plain CSS functions don\'t support keyword arguments.\
         \n  ,\
         \n2 |     color: missing($a: b);\
         \n  |            ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
    );
}
