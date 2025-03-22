//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/067_test_mixin_splat_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("067_test_mixin_splat_args")
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
             \n$list: 2, 3, 4;\
             \n.foo {@include foo(1, $list...)}\n"),
        ".foo {\
         \n  a: 1;\
         \n  b: 2;\
         \n  c: 3;\
         \n  d: 4;\
         \n}\n"
    );
}
