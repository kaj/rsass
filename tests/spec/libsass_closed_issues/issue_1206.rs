//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1206.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: #{0/0};\
            \n  bar: #{0/1};\
            \n  bar: #{1/2};\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: 0/0;\
        \n  bar: 0/1;\
        \n  bar: 1/2;\
        \n}\
        \n"
    );
}
