//! Tests auto-converted from "sass-spec/spec/non_conformant/parser"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/and_and.hrx"
#[test]
fn and_and() {
    assert_eq!(
        rsass(
            ".and-and {\
            \n  value: true && false;\
            \n}\
            \n"
        )
        .unwrap(),
        ".and-and {\
        \n  value: true .and-and .and-and false;\
        \n}\
        \n"
    );
}

mod arglists;

mod interpolate;

mod malformed_expressions;

mod operations;
