//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-prefix-itpl.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "pre#{&} {\r\
            \n  foo {\r\
            \n    bar: baz;\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "pre foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}
