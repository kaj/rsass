//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/cfunc.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  blah: say-something();\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: say-something();\
        \n}\
        \n"
    );
}
