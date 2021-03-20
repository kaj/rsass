//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1945.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: #{\"\\\\\"}#{\"baz\"};\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: \\baz;\
        \n}\
        \n"
    );
}
