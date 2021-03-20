//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/multiple-operators.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: 2;\
            \n$y: 1;\
            \n\
            \n@function getResult() { @return true; }\
            \n\
            \n.test {\
            \n    a: $x > $y == getResult();\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  a: true;\
        \n}\
        \n"
    );
}
