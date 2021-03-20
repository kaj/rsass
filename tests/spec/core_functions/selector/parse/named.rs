//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse/named.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-parse($selector: \"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
