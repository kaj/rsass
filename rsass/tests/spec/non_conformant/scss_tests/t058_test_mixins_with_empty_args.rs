//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/058_test_mixins_with_empty_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("058_test_mixins_with_empty_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo() {a: b}\n\
             \n.foo {@include foo;}\n"),
        ".foo {\
         \n  a: b;\
         \n}\n"
    );
}
