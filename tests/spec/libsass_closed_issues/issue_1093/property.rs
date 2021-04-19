//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/property.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
             \n  bar: #{};\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n2 |   bar: #{};\
         \n  |        ^^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
