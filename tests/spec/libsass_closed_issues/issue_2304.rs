//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2304.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"module\";"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | .foo, & {\
         \n  | ^^^^^^^^\
         \n  \'\
         \n  _module.scss 1:1  @import\
         \n  input.scss 1:9    root stylesheet\
         \n",
    );
}
