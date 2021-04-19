//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-postfix.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"include.scss\";"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | &post {\
         \n  | ^^^^^^\
         \n  \'\
         \n  include.scss 1:1  @import\
         \n  input.scss 1:9    root stylesheet\
         \n",
    );
}
