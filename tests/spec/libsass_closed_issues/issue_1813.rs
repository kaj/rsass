//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1813.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($value) {\
            \n  $a: bar($value);\
            \n  @return $value;\
            \n}\
            \n\
            \n@function bar($list) {\
            \n  @while (true) {\
            \n    @return true;\
            \n  }\
            \n}\
            \n\
            \na {\
            \n  b: foo(true);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
