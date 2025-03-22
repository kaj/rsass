//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/053_test_no_namespace_properties_without_space.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("053_test_no_namespace_properties_without_space")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar:baz {\
             \n    bip: bop }}\n"),
        "foo bar:baz {\
         \n  bip: bop;\
         \n}\n"
    );
}
