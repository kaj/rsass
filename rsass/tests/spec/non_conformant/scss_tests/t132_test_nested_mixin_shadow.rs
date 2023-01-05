//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/132_test_nested_mixin_shadow.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("132_test_nested_mixin_shadow")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin bar {a: b}\n\
             \nfoo {\
             \n  @mixin bar {c: d}\
             \n  @include bar;\
             \n}\n\
             \nbaz {@include bar}\n"),
        "foo {\
         \n  c: d;\
         \n}\
         \nbaz {\
         \n  a: b;\
         \n}\n"
    );
}
