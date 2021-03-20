//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-debug/ruleset.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
