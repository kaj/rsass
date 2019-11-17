//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1651/with.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
             \n  display: block;\
             \n}\
             \n.b {\
             \n  @at-root (with: media) {\
             \n    @extend .a;\
             \n  }\
             \n} \
             \n"
        )
        .unwrap_err(),
        "Error: @extend may only be used within style rules.\
         \n  ,\
         \n6 |     @extend .a;\
         \n  |     ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 6:5  root stylesheet\
         \n",
    );
}
