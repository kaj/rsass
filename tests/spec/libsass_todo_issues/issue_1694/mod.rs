//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1694"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-todo-issues/issue_1694/quoted-right-dbl-paren.hrx"
#[test]
#[ignore] // unexepected error
fn quoted_right_dbl_paren() {
    assert_eq!(
        rsass(
            "test {\
            \n  filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20\\));\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20\\));\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-todo-issues/issue_1694/quoted-right-paren.hrx"
#[test]
fn quoted_right_paren() {
    assert_eq!(
        rsass(
            "test {\
            \n  filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20\\);\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20\\);\
        \n}\
        \n"
    );
}
