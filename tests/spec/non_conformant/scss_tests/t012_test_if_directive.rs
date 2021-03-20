//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/012_test_if_directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@if \"foo\" != \"foo\" {foo {a: b}}\
            \n@else if \"foo\" == \"foo\" {bar {a: b}}\
            \n@else if true {baz {a: b}}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
