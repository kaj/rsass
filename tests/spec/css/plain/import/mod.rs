//! Tests auto-converted from "sass-spec/spec/css/plain/import"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/css/plain/import/css_before_index.hrx"
#[test]
#[ignore] // wrong result
fn css_before_index() {
    assert_eq!(
        rsass(
            "@import \'other\';\
            \n"
        )
        .unwrap(),
        "other {\
        \n  index: false;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/plain/import/in_css.hrx"
#[test]
#[ignore] // wrong result
fn in_css() {
    assert_eq!(
        rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "@import \"whatever\";\
        \n"
    );
}

// From "sass-spec/spec/css/plain/import/partial_conflict.hrx"

// Ignoring "partial_conflict", error tests are not supported yet.

// From "sass-spec/spec/css/plain/import/sass_takes_precedence.hrx"
#[test]
#[ignore] // wrong result
fn sass_takes_precedence() {
    assert_eq!(
        rsass(
            "@import \"other\";\
            \n"
        )
        .unwrap(),
        "other {\
        \n  syntax: sass;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/plain/import/scss_takes_precedence.hrx"
#[test]
#[ignore] // wrong result
fn scss_takes_precedence() {
    assert_eq!(
        rsass(
            "@import \"other\";\
            \n"
        )
        .unwrap(),
        "other {\
        \n  syntax: scss;\
        \n}\
        \n"
    );
}
