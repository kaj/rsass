//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-postfix-itpl.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "#{&}post {\r\
            \n  foo {\r\
            \n    bar: baz;\r\
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
