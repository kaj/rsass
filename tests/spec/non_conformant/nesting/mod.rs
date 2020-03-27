//! Tests auto-converted from "sass-spec/spec/non_conformant/nesting"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/nesting/not.hrx"
mod not {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn multiple_parent_selectors_with_trailing_ident() {
        assert_eq!(
            rsass(
                "// Regression test for sass/libsass#2630\
            \n.a, .b {\
            \n  :not(&-c) {d: e}\
            \n}\
            \n"
            )
            .unwrap(),
            ":not(.a-c, .b-c) {\
        \n  d: e;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/non_conformant/nesting/parent_with_newline.hrx"
#[test]
#[ignore] // wrong result
fn parent_with_newline() {
    assert_eq!(
        rsass(
            ".foo,\
            \n.bar {\
            \n  .baz & {\
            \n    color: red;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".baz .foo,\
        \n.baz .bar {\
        \n  color: red;\
        \n}\
        \n"
    );
}
