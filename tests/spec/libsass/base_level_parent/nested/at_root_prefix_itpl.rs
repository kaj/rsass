//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/at-root-prefix-itpl.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
            \n  @at-root {\r\
            \n    pre#{&} {\r\
            \n      foo {\r\
            \n        bar: baz;\r\
            \n      }\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "pretest foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}
