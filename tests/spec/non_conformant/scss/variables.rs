//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/variables.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  $var: 2;\
            \n  $another-var: 4;\
            \n  a: $var;\
            \n  b: $var + $another-var;}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 2;\
        \n  b: 6;\
        \n}\
        \n"
    );
}
