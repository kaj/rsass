//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-postfix-itpl.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@at-root {\r\
            \n  #{&}post {\r\
            \n    foo {\r\
            \n      bar: baz;\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "post foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}
