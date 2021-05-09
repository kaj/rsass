//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/131_test_nested_mixin_def.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  @mixin bar {a: b}\
             \n  @include bar; }\n"),
        "foo {\
         \n  a: b;\
         \n}\n"
    );
}
