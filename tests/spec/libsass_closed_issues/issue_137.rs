//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_137.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  background-color: lime;\
            \n  a {\
            \n    color: white;\
            \n  }\
            \n}\
            \n\
            \n.baz {\
            \n  @extend .foo;\
            \n}"
        )
        .unwrap(),
        ".foo, .baz {\
        \n  background-color: lime;\
        \n}\
        \n.foo a, .baz a {\
        \n  color: white;\
        \n}\
        \n"
    );
}
