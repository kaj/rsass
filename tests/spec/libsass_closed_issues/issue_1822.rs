//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1822.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            ".btn {\
             \n    .open& {\
             \n        color: #000;\
             \n    }\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n2 |     .open&{\
         \n  |          ^\
         \n  \'\
         \n  input.scss 2:10  root stylesheet\
         \n",
    );
}
