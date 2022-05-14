//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/061_test_mixins_with_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("061_test_mixins_with_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($a, $b) {\
             \n  a: $a;\
             \n  b: $b; }\n\
             \n.foo {@include foo(bar, 12px)}\n"),
        ".foo {\
         \n  a: bar;\
         \n  b: 12px;\
         \n}\n"
    );
}
