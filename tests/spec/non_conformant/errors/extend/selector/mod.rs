//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/extend/selector"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/errors/extend/selector/missing.hrx"

// Ignoring "missing", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/errors/extend/selector/optional.hrx"
#[test]
#[ignore] // wrong result
fn optional() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/errors/extend/selector/simple.hrx"
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        rsass(
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
