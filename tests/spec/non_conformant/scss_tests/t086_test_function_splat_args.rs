//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/086_test_function_splat_args.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($a, $b, $c, $d) {\
            \n  @return \"a: #{$a}, b: #{$b}, c: #{$c}, d: #{$d}\";\
            \n}\
            \n\
            \n$list: 2, 3, 4;\
            \n.foo {val: foo(1, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 1, b: 2, c: 3, d: 4\";\
        \n}\
        \n"
    );
}
