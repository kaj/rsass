//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-alone-itpl.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass("@import \"include.scss\";").unwrap_err(),
        "Error: expected selector.\
         \n  ,\
         \n1 | {\
         \n  | ^\
         \n  \'\
         \n  include.scss 1:1  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
