//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/085_test_function_var_args_act_like_list.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($a, $b...) {\
            \n  @return \"a: #{length($b)}, b: #{nth($b, 2)}\";\
            \n}\
            \n\
            \n.foo {val: foo(1, 2, 3, 4)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 3, b: 3\";\
        \n}\
        \n"
    );
}
