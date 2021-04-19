//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-postfix.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "&post {\
             \n  foo {\
             \n    bar: baz;\
             \n  }\
             \n}"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | &post {\
         \n  | ^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
