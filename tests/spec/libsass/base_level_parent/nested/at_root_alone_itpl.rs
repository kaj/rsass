//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/at-root-alone-itpl.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
            \n  @at-root {\r\
            \n    #{&} {\r\
            \n      foo {\r\
            \n        bar: baz;\r\
            \n      }\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "test foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}
