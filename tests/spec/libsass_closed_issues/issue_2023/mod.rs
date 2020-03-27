//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2023"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_2023/id-selector-id.hrx"
#[test]
fn id_selector_id() {
    assert_eq!(
        rsass(
            "#2b {\r\
            \n    color: red;\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "#2b {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2023/id-selector-nr.hrx"
#[test]
fn id_selector_nr() {
    assert_eq!(
        rsass(
            "#2 {\r\
            \n    color: red;\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "#2 {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2023/pseudo-selector-id.hrx"
#[test]
fn pseudo_selector_id() {
    assert_eq!(
        rsass(
            "#4d {\r\
            \n    color: red;\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "#4d {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2023/pseudo-selector-nr.hrx"
#[test]
fn pseudo_selector_nr() {
    assert_eq!(
        rsass(
            "#4 {\r\
            \n    color: red;\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "#4 {\
        \n  color: red;\
        \n}\
        \n"
    );
}
