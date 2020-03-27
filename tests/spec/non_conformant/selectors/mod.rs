//! Tests auto-converted from "sass-spec/spec/non_conformant/selectors"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/selectors/attribute.hrx"
mod attribute {
    #[allow(unused)]
    use super::rsass;
    mod regression {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn trailing_dot() {
            assert_eq!(
                rsass(
                    "// See https://github.com/sass/dart-sass/issues/598\
            \n[a=\"b.\"] {x: y}\
            \n"
                )
                .unwrap(),
                "[a=\"b.\"] {\
        \n  x: y;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/non_conformant/selectors/placeholder_in_pseudo.hrx"
#[test]
#[ignore] // wrong result
fn placeholder_in_pseudo() {
    assert_eq!(
        rsass(
            "// Style rules with placeholders should only be deleted if they can\'t match\
            \n// *any* real elements.\
            \na:matches(%b) {x: y}\
            \na:matches(%b, c) {x: y}\
            \na:not(%b) {x: y}\
            \na:not(%b, c) {x: y}\
            \n:not(%b) {x: y}\
            \n"
        )
        .unwrap(),
        "a:matches(c) {\
        \n  x: y;\
        \n}\
        \na {\
        \n  x: y;\
        \n}\
        \na:not(c) {\
        \n  x: y;\
        \n}\
        \n* {\
        \n  x: y;\
        \n}\
        \n"
    );
}
