//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_151.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "div.colors {\
             \n  background: grayscale(red);\
             \n  background: saturate(red);\
             \n  background: invert(red);\
             \n  background: alpha(red);\
             \n  color: grayscale(#369);\
             \n  color: saturate(#369);\
             \n  color: saturate(#369, 20%);\
             \n  color: invert(#369);\
             \n  color: alpha(#369);\
             \n}\
             \n\
             \ndiv.numbers {\
             \n  filter: grayscale(30%);\
             \n  filter: saturate(30%);\
             \n  filter: invert(30%);\
             \n  -webkit-filter: grayscale(0.3);\
             \n  -webkit-filter: saturate(0.3);\
             \n  -webkit-filter: invert(0.3);\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: $amount: red is not a number.\
         \n  ,\
         \n3 |   background: saturate(red);\
         \n  |               ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:15  root stylesheet\
         \n",
    );
}
