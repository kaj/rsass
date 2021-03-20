//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-warn/ruleset.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
