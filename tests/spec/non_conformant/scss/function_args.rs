//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/function_args.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function plus($var1, $var2) {\
            \n  @return $var1 + $var2;\
            \n}\
            \n\
            \nbar {\
            \n  a: plus(1, 2);\
            \n}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: 3;\
        \n}\
        \n"
    );
}
