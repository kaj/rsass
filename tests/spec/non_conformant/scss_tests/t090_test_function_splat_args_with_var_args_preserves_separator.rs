//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/090_test_function_splat_args_with_var_args_preserves_separator.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($a, $b...) {\
            \n  @return \"a: #{$a}, b: #{$b}\";\
            \n}\
            \n\
            \n$list: 3 4 5;\
            \n.foo {val: foo(1, 2, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 1, b: 2 3 4 5\";\
        \n}\
        \n"
    );
}
