//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/namespace_properties_with_value.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("namespace_properties_with_value")
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
