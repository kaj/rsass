//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/013_test_dynamic_extendee.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "[baz^=\"blip12px\"] {a: b}\
            \n.bar {@extend [baz^=\"blip#{12px}\"]}\
            \n"
        )
        .unwrap(),
        "[baz^=blip12px], .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
