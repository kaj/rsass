//! Tests auto-converted from "sass-spec/spec/directives/if"
#[allow(unused)]
use super::rsass;

mod error;

// From "sass-spec/spec/directives/if/escaped.hrx"
mod escaped {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn if_only() {
        assert_eq!(
        rsass(
            "// Escapes should be normalized before directives are parsed.\
            \n@\\69 f true {a {b: c}}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn with_else() {
        assert_eq!(
            rsass(
                "// See sass/dart-sass#1011\
            \n@if false {}\
            \n@\\65lse {a {b: c}}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
}
