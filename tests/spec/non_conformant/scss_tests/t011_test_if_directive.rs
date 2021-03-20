//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/011_test_if_directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@if \"foo\" == \"foo\" {foo {a: b}}\
            \n@if \"foo\" != \"foo\" {bar {a: b}}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
