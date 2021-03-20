//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-debug/property.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
