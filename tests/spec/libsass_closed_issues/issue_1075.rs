//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1075.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$name: \"lighten\";\
            \n$args: (\"color\": #ff0000, \"amount\": 10%);\
            \nfoo {\
            \n  bar: call($name, $args...);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: #ff3333;\
        \n}\
        \n"
    );
}
