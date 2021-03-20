//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_557.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "\
            \na {\
            \n  foo: map-get((foo: 1, bar: 2), \"bar\");\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: 2;\
        \n}\
        \n"
    );
}
