//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1394.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  width: \\10 + \\20 \\ ;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  width: \\10 \\ \\ ;\
        \n}\
        \n"
    );
}
