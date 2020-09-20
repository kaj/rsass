//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-debug"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/errors/fn-debug/property.hrx"
#[test]
#[ignore] // wrong result
fn property() {
    assert_eq!(
        rsass(
            "a {\r\
            \n  b: {\r\
            \n    @debug \"debug\";\r\
            \n    foo: bar;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  b-foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/errors/fn-debug/ruleset.hrx"
#[test]
#[ignore] // wrong result
fn ruleset() {
    assert_eq!(
        rsass(
            "a {\r\
            \n  @debug \"debug\";\r\
            \n  foo: bar;\r\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/errors/fn-debug/simple.hrx"
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(rsass("@debug \"debug\";").unwrap(), "");
}
