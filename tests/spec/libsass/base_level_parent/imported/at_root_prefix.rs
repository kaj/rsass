//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/at-root-prefix.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass("@import \"include.scss\";").unwrap_err(),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n2 |   pre&{\
         \n  |      ^\
         \n  \'\
         \n  include.scss 2:6  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
