//! Tests auto-converted from "sass-spec/spec/libsass/precision"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/libsass/precision/default.hrx"
#[test]
#[ignore] // failing
fn default() {
    assert_eq!(
        rsass(
            "test {\r\n  foo: 0.4999 round(0.4999);\r\n  bar: 0.49999 round(0.49999);\r\n  baz: 0.499999 round(0.499999);\r\n  baz: 0.49999999999 round(0.49999999999);\r\n}\r\n"
        )
        .unwrap(),
        "test {\n  foo: 0.4999 0;\n  bar: 0.49999 0;\n  baz: 0.499999 0;\n  baz: 0.5 0;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/precision/higher.hrx"
#[test]
#[ignore] // failing
fn higher() {
    set_precision(6);
    assert_eq!(
        rsass(
            "test {\r\n  foo: 0.4999 round(0.4999);\r\n  bar: 0.49999 round(0.49999);\r\n  baz: 0.499999 round(0.499999);\r\n}"
        )
        .unwrap(),
        "test {\n  foo: 0.4999 0;\n  bar: 0.49999 0;\n  baz: 0.499999 0;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/precision/lower.hrx"
#[test]
#[ignore] // failing
fn lower() {
    set_precision(4);
    assert_eq!(
        rsass(
            "test {\r\n  foo: 0.4999 round(0.4999);\r\n  bar: 0.49999 round(0.49999);\r\n  baz: 0.499999 round(0.499999);\r\n}"
        )
        .unwrap(),
        "test {\n  foo: 0.4999 0;\n  bar: 0.5 0;\n  baz: 0.5 1;\n}\n"
    );
}
