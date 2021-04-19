//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-alone.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "& {\
             \n  foo {\
             \n    bar: baz;\
             \n  }\
             \n}\
             \n"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | & {\
         \n  | ^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
