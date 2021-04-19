//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1601.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            ".code.ruby > {\r\
             \n    &.ruby {\r\
             \n        color: green;\r\
             \n    }\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Parent \".code.ruby >\" is incompatible with this selector.\
         \n  ,\
         \n2 |     &.ruby {\
         \n  |     ^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
    );
}
