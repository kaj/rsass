//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/04_basic_variables.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$color: \"black\";\
            \n$color: red;\
            \n$background: \"blue\";\
            \n\
            \na {\
            \n  color: $color;\
            \n  background: $background;\
            \n}\
            \n\
            \n$y: before;\
            \n\
            \n$x: 1 2 $y;\
            \n\
            \nfoo {\
            \n  a: $x;\
            \n}\
            \n\
            \n$y: after;\
            \n\
            \nfoo {\
            \n  a: $x;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  color: red;\
        \n  background: \"blue\";\
        \n}\
        \nfoo {\
        \n  a: 1 2 before;\
        \n}\
        \nfoo {\
        \n  a: 1 2 before;\
        \n}\
        \n"
    );
}
