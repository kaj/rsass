//! Tests auto-converted from "sass-spec/spec/non_conformant/nesting/parent_with_newline.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo,\
            \n.bar {\
            \n  .baz & {\
            \n    color: red;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".baz .foo,\
        \n.baz .bar {\
        \n  color: red;\
        \n}\
        \n"
    );
}
