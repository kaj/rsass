//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/051_test_namespace_properties_with_value.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("051_test_namespace_properties_with_value")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: baz {\
             \n    bip: bop;\
             \n    bing: bop; }}\n"),
        "foo {\
         \n  bar: baz;\
         \n  bar-bip: bop;\
         \n  bar-bing: bop;\
         \n}\n"
    );
}
