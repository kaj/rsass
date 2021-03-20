//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_506.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$list: foo bar baz;\
            \n$list--comma: foo, bar, baz;\
            \n$single: foo;\
            \n\
            \ndiv {\
            \n  _list-space: list-separator($list);\
            \n  _list-comma: list-separator($list--comma);\
            \n  _single-item: list-separator($single);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  _list-space: space;\
        \n  _list-comma: comma;\
        \n  _single-item: space;\
        \n}\
        \n"
    );
}
