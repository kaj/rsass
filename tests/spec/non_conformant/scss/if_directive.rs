//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/if_directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@if \"foo\" != \"foo\" {foo {a: b}}\
            \n@else {bar {a: b}}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
