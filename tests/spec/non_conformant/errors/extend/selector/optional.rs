//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/extend/selector/optional.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".baz {\r\
            \n  @extend .foo !optional;\r\
            \n  color: green;\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        ".baz {\
        \n  color: green;\
        \n}\
        \n"
    );
}
