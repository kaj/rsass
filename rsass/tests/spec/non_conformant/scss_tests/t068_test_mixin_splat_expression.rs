//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/068_test_mixin_splat_expression.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("068_test_mixin_splat_expression")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($a, $b, $c, $d) {\
             \n  a: $a;\
             \n  b: $b;\
             \n  c: $c;\
             \n  d: $d;\
             \n}\n\
             \n.foo {@include foo(1, (2, 3, 4)...)}\n"),
        ".foo {\
         \n  a: 1;\
         \n  b: 2;\
         \n  c: 3;\
         \n  d: 4;\
         \n}\n"
    );
}
