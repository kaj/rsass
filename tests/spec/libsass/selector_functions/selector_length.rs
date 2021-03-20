//! Tests auto-converted from "sass-spec/spec/libsass/selector-functions/selector-length.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo.bar.baz asd.qwe xyz, second {\r\
            \n  length: length(&);\r\
            \n  length: length(nth(&, 1));\r\
            \n  length: length(nth(nth(&, 1), 1));\r\
            \n}"
        )
        .unwrap(),
        "foo.bar.baz asd.qwe xyz, second {\
        \n  length: 2;\
        \n  length: 3;\
        \n  length: 1;\
        \n}\
        \n"
    );
}
