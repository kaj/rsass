//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/mixin-content-with-no-block.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo {\
            \n  .foo {\
            \n    color: red;\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \ndiv.a {\
            \n  @include foo() {\
            \n    hey: now;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div.a .foo {\
        \n  color: red;\
        \n  hey: now;\
        \n}\
        \n"
    );
}
