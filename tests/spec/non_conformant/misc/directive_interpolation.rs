//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/directive_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$baz: 12;\
            \n@foo bar#{$baz} qux {a: b}\
            \n"
        )
        .unwrap(),
        "@foo bar12 qux {\
        \n  a: b;\
        \n}\
        \n"
    );
}
