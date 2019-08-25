//! Tests auto-converted from "sass-spec/spec/libsass/selector-functions"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/libsass/selector-functions/selector-append-empty.hrx"

// Ignoring "selector_append_empty", error tests are not supported yet.

// From "sass-spec/spec/libsass/selector-functions/selector-append-error-1.hrx"

// Ignoring "selector_append_error_1", error tests are not supported yet.

// From "sass-spec/spec/libsass/selector-functions/selector-append-error-2.hrx"

// Ignoring "selector_append_error_2", error tests are not supported yet.

// From "sass-spec/spec/libsass/selector-functions/selector-length.hrx"
#[test]
fn selector_length() {
    assert_eq!(
        rsass(
            "foo.bar.baz asd.qwe xyz, second {\r\n  length: length(&);\r\n  length: length(nth(&, 1));\r\n  length: length(nth(nth(&, 1), 1));\r\n}"
        )
        .unwrap(),
        "foo.bar.baz asd.qwe xyz, second {\n  length: 2;\n  length: 3;\n  length: 1;\n}\n"
    );
}

// From "sass-spec/spec/libsass/selector-functions/simple-selector.hrx"
#[test]
#[ignore] // failing
fn simple_selector() {
    assert_eq!(
        rsass(
            "foo {\r\n  test-01: simple-selectors(\".foo.bar\");\r\n  test-02: simple-selectors(\".foo.bar.baz\");\r\n}"
        )
        .unwrap(),
        "foo {\n  test-01: .foo, .bar;\n  test-02: .foo, .bar, .baz;\n}\n"
    );
}
