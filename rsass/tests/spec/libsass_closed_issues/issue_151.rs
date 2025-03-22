//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_151.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_151")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \ndiv.colors {\
             \n  background: color.grayscale(red);\
             \n  background: saturate(red);\
             \n  background: color.invert(red);\
             \n  background: color.alpha(red);\
             \n  color: color.grayscale(#369);\
             \n  color: saturate(#369);\
             \n  color: color.adjust(#369, $saturation: 20%);\
             \n  color: color.invert(#369);\
             \n  color: color.alpha(#369);\
             \n}\n\
             \ndiv.numbers {\
             \n  filter: grayscale(30%);\
             \n  filter: saturate(30%);\
             \n  filter: invert(30%);\
             \n  -webkit-filter: grayscale(0.3);\
             \n  -webkit-filter: saturate(0.3);\
             \n  -webkit-filter: invert(0.3);\
             \n}\n"
        ),
        "Error: $amount: red is not a number.\
         \n  ,\
         \n4 |   background: saturate(red);\
         \n  |               ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:15  root stylesheet",
    );
}
