//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/049_test_several_namespace_properties.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("049_test_several_namespace_properties")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: baz;\
             \n  bang: {\
             \n    bip: 1px;\
             \n    bop: bar;}\
             \n  buzz: {\
             \n    fram: \"foo\";\
             \n    frum: moo;\
             \n  }\
             \n}\n"),
        "foo {\
         \n  bar: baz;\
         \n  bang-bip: 1px;\
         \n  bang-bop: bar;\
         \n  buzz-fram: \"foo\";\
         \n  buzz-frum: moo;\
         \n}\n"
    );
}
