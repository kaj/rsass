//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/060_test_mixins_with_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("060_test_mixins_with_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($a) {a: $a}\n\
             \n.foo {@include foo(bar)}\n"),
        ".foo {\
         \n  a: bar;\
         \n}\n"
    );
}
