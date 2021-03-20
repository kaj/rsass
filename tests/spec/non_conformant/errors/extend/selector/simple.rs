//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/extend/selector/simple.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {color: blue}\r\
            \n.bar {color: red}\r\
            \n.baz {\r\
            \n  @extend .foo;\r\
            \n  color: green;\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        ".foo, .baz {\
        \n  color: blue;\
        \n}\
        \n.bar {\
        \n  color: red;\
        \n}\
        \n.baz {\
        \n  color: green;\
        \n}\
        \n"
    );
}
