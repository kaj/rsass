//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-todo-issues/issue_1026.hrx"
#[test]
#[ignore] // wrong result
fn issue_1026() {
    assert_eq!(
        rsass(
            "div {\
            \n  a {\
            \n    /**\
            \n     * a\
            \n     * multiline\
            \n     * comment\
            \n     */\
            \n    top: 10px;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div a {\
        \n  /**\
        \n   * a\
        \n   * multiline\
        \n   * comment\
        \n   */\
        \n  top: 10px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-todo-issues/issue_1096.hrx"
#[test]
#[ignore] // wrong result
fn issue_1096() {
    assert_eq!(
        rsass(
            "// line-endings in this file must be CRLF\r\
            \n@import url(\"foo\\\r\
            \nbar\");\r\
            \n@import url(\"foo\r\
            \nbar\");\r\
            \n@import url(foo\r\
            \nbar);\r\
            \n"
        )
        .unwrap(),
        "@import url(\"foobar\");\
        \n@import url(\"foo\\a bar\");\
        \n@import url(foo bar);\
        \n"
    );
}

mod issue_1694;

mod issue_1732;

// From "sass-spec/spec/libsass-todo-issues/issue_1763.hrx"
#[test]
#[ignore] // wrong result
fn issue_1763() {
    assert_eq!(
        rsass(
            "@import \"first.css\", \"second.css\" (max-width: 400px);\
            \n@import \"first.scss\", \"second.scss\" (max-width: 400px);\
            \n"
        )
        .unwrap(),
        "@import \"first.css\";\
        \n@import \"second.css\" (max-width: 400px);\
        \n@import \"second.scss\" (max-width: 400px);\
        \nfoo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

mod issue_1798;

mod issue_1801;

// Ignoring "issue_2016", tests with expected error not implemented yet.

mod issue_2023;

// From "sass-spec/spec/libsass-todo-issues/issue_2051.hrx"

// Ignoring "issue_2051", error tests are not supported yet.

// From "sass-spec/spec/libsass-todo-issues/issue_2096.hrx"
#[test]
fn issue_2096() {
    assert_eq!(
        rsass(
            "@mixin foo() {\
            \n  @import \"https://foo\";\
            \n}\
            \n@include foo;\
            \n"
        )
        .unwrap(),
        "@import \"https://foo\";\
        \n"
    );
}

// From "sass-spec/spec/libsass-todo-issues/issue_221260.hrx"

// Ignoring "issue_221260", error tests are not supported yet.

// Ignoring "issue_221262.hrx", not expected to work yet.

// Ignoring "issue_221264", tests with expected error not implemented yet.

// Ignoring "issue_221267", tests with expected error not implemented yet.

// Ignoring "issue_221286", tests with expected error not implemented yet.

// Ignoring "issue_221292.hrx", not expected to work yet.

mod issue_2235;

mod issue_2295;

// From "sass-spec/spec/libsass-todo-issues/issue_238764.hrx"

// Ignoring "issue_238764", error tests are not supported yet.

// Ignoring "issue_245442", tests with expected error not implemented yet.

// Ignoring "issue_245446", tests with expected error not implemented yet.

// From "sass-spec/spec/libsass-todo-issues/issue_2818.hrx"
#[test]
#[ignore] // unexepected error
fn issue_2818() {
    assert_eq!(
        rsass(
            "$map: (\"lightness\": 10%, \"saturation\": 10%);\
            \n$base: call(get-function(\'scale-color\'), #dedede, $map...);\
            \ntest { color: $base; }\
            \n"
        )
        .unwrap(),
        "test {\
        \n  color: #e4dede;\
        \n}\
        \n"
    );
}
