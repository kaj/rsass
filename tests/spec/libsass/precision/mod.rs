//! Tests auto-converted from "sass-spec/spec/libsass/precision"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass/precision/default.hrx"
#[test]
fn default() {
    assert_eq!(
        rsass(
            "test {\r\
            \n  foo: 0.4999 round(0.4999);\r\
            \n  bar: 0.49999 round(0.49999);\r\
            \n  baz: 0.499999 round(0.499999);\r\
            \n  baz: 0.49999999999 round(0.49999999999);\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "test {\
        \n  foo: 0.4999 0;\
        \n  bar: 0.49999 0;\
        \n  baz: 0.499999 0;\
        \n  baz: 0.5 0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/precision/higher.hrx"
#[test]
fn higher() {
    let format = rsass::output::Format {
        style: rsass::output::Style::Expanded,
        precision: 6,
    };
    assert_eq!(
        crate::rsass_fmt(
            format,
            "test {\r\
            \n  foo: 0.4999 round(0.4999);\r\
            \n  bar: 0.49999 round(0.49999);\r\
            \n  baz: 0.499999 round(0.499999);\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  foo: 0.4999 0;\
        \n  bar: 0.49999 0;\
        \n  baz: 0.499999 0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/precision/lower.hrx"
#[test]
#[ignore] // wrong result
fn lower() {
    let format = rsass::output::Format {
        style: rsass::output::Style::Expanded,
        precision: 4,
    };
    assert_eq!(
        crate::rsass_fmt(
            format,
            "test {\r\
            \n  foo: 0.4999 round(0.4999);\r\
            \n  bar: 0.49999 round(0.49999);\r\
            \n  baz: 0.499999 round(0.499999);\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  foo: 0.4999 0;\
        \n  bar: 0.49999 0;\
        \n  baz: 0.499999 0;\
        \n}\
        \n"
    );
}
