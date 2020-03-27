//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1681"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1681/calc.hrx"
#[test]
fn calc() {
    assert_eq!(
        rsass(
            "@function calc() {\
            \n  @return null;\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1681/element.hrx"
#[test]
fn element() {
    assert_eq!(
        rsass(
            "@function element() {\
            \n  @return null;\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1681/expression.hrx"
#[test]
fn expression() {
    assert_eq!(
        rsass(
            "@function expression() {\
            \n  @return null;\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1681/url.hrx"
#[test]
fn url() {
    assert_eq!(
        rsass(
            "@function url() {\
            \n  @return null;\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
