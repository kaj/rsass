//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1081.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: foo !global !default;\
            \n\
            \ndefault {\
            \n  foo: $foo;\
            \n}\
            \n\
            \n$foo: bar;\
            \n\
            \nafter {\
            \n  @import \"import\";\
            \n  foo: $foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "default {\
        \n  foo: foo;\
        \n}\
        \nafter {\
        \n  foo: bar;\
        \n}\
        \nafter import-before {\
        \n  foo: bar;\
        \n}\
        \nafter import-after {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
