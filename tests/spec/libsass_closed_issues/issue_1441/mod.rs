//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1441"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1441/adjacent.hrx"
#[test]
#[ignore] // wrong result
fn adjacent() {
    assert_eq!(
        rsass(
            ".adjacent {\
            \n    & + & {\
            \n        foo: bar;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        ".adjacent + .adjacent {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1441/child.hrx"
#[test]
#[ignore] // wrong result
fn child() {
    assert_eq!(
        rsass(
            ".child {\
            \n    & > & {\
            \n        foo: bar;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        ".child > .child {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1441/sibling.hrx"
#[test]
#[ignore] // wrong result
fn sibling() {
    assert_eq!(
        rsass(
            ".sibling {\
            \n    & ~ & {\
            \n        foo: bar;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        ".sibling ~ .sibling {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
