//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-prefix-itpl.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@at-root {\r\
            \n  pre#{&} {\r\
            \n    foo {\r\
            \n      bar: baz;\r\
            \n    }\r\
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
