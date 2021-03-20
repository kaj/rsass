//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/38_expressions_in_at_directives.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: 1;\
            \n$y: 2;\
            \n\
            \n@foo $x $y, hux {\
            \n  bar {\
            \n    whatever: whatever;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@foo $x $y, hux {\
        \n  bar {\
        \n    whatever: whatever;\
        \n  }\
        \n}\
        \n"
    );
}
