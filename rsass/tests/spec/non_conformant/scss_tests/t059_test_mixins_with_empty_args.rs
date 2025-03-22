//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/059_test_mixins_with_empty_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("059_test_mixins_with_empty_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo {a: b}\n\
             \n.foo {@include foo();}\n"),
        ".foo {\
         \n  a: b;\
         \n}\n"
    );
}
