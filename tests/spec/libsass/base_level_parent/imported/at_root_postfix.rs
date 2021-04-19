//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/at-root-postfix.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"include.scss\";"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n2 |   &post {\
         \n  |   ^^^^^^\
         \n  \'\
         \n  include.scss 2:3  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
