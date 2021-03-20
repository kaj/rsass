//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1669.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: #{100%/3}\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: 100%/3;\
        \n}\
        \n"
    );
}
