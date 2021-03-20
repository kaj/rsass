//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/JMA-pseudo-test.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n        h1 {\
            \n                color:red;\
            \n        }\
            \n}\
            \n\
            \n.bar {\
            \n        &:hover h3,\
            \n        h3 {\
            \n                @extend h1;\
            \n        }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo h1,\
        \n.foo .bar h3,\
        \n.bar .foo h3 {\
        \n  color: red;\
        \n}\
        \n"
    );
}
