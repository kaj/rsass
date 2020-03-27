//! Tests auto-converted from "sass-spec/spec/directives/if"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/directives/if/escaped.hrx"
#[test]
#[ignore] // wrong result
fn escaped() {
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
