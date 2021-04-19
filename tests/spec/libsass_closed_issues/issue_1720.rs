//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1720.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            ".test {\
             \n    a: a#{b//}c;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: expected \"}\".\
         \n  ,\
         \n3 | }\
         \n  |  ^\
         \n  \'\
         \n  input.scss 3:2  root stylesheet\
         \n",
    );
}
