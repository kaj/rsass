//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/at-root-alone-itpl.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass("@import \"include.scss\";").unwrap_err(),
        "Error: expected selector.\
         \n  ,\
         \n2 |   {\
         \n  |   ^\
         \n  \'\
         \n  include.scss 2:3  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
