//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/064_test_mixin_var_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($a, $b...) {\
             \n  a: $a;\
             \n  b: $b;\
             \n}\n\
             \n.foo {@include foo(1, 2, 3, 4)}\n"),
        ".foo {\
         \n  a: 1;\
         \n  b: 2, 3, 4;\
         \n}\n"
    );
}
