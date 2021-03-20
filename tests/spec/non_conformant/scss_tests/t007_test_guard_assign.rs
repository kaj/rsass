//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/007_test_guard_assign.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$var: 2 !default;\
            \n\
            \nfoo {a: $var}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 2;\
        \n}\
        \n"
    );
}
