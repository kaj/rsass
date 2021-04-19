//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2175.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "a:nth-child() {\
             \n  color: yellowgreen;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Expected \"n\".\
         \n  ,\
         \n1 | a:nth-child(){\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
