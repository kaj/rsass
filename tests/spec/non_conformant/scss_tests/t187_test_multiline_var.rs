//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/187_test_multiline_var.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  $var1: 1 +\
            \n    2;\
            \n  $var2: true and\
            \n    false;\
            \n  $var3: a b\
            \n    c;\
            \n  a: $var1;\
            \n  b: $var2;\
            \n  c: $var3; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 3;\
        \n  b: false;\
        \n  c: a b c;\
        \n}\
        \n"
    );
}
