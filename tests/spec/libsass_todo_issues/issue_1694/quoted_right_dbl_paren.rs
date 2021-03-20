//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1694/quoted-right-dbl-paren.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
