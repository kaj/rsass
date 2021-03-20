//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_644.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  background-image: url(foo/#{\"bar\"}/baz.jpg);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  background-image: url(foo/bar/baz.jpg);\
        \n}\
        \n"
    );
}
