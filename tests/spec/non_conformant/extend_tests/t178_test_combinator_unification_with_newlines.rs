//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/178_test_combinator_unification_with_newlines.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a >\
            \n.b\
            \n+ x {a: b}\
            \n.c\
            \n> .d +\
            \ny {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > .b + x, .c.a > .d.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}
