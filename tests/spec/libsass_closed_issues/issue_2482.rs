//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2482.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin mixin()\
             \n{\
             \n\t& .class\
             \n\t{\
             \n\t\tcolor: black;\
             \n\t}\
             \n}\
             \n\
             \n@include mixin();"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n3 | /     & .class\
         \n4 | |     {\
         \n  | \'----^\
         \n  \'\
         \n  input.scss 3:2  mixin()\
         \n  input.scss 9:1  root stylesheet",
    );
}
