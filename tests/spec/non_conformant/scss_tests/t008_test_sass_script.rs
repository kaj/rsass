//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/008_test_sass_script.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  a: 1 + 2;\
            \n  b: 1 - 2;\
            \n  c: foo + bar;\
            \n  d: floor(12.3px); }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 3;\
        \n  b: -1;\
        \n  c: foobar;\
        \n  d: 12px;\
        \n}\
        \n"
    );
}
