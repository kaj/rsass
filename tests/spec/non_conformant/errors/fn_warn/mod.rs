//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-warn"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/errors/fn-warn/property.hrx"

// Ignoring "property", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/errors/fn-warn/ruleset.hrx"
#[test]
fn ruleset() {
    assert_eq!(
        rsass(
            "a {\r\
            \n  @warn \"warn\";\r\
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

// From "sass-spec/spec/non_conformant/errors/fn-warn/simple.hrx"
#[test]
fn simple() {
    assert_eq!(rsass("@warn \"warn\";").unwrap(), "");
}
