//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/namespace_properties.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("namespace_properties")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: baz;\
             \n  bang: {\
             \n    bip: 1px;\
             \n    bop: bar;}}\n"),
        "foo {\
         \n  bar: baz;\
         \n  bang-bip: 1px;\
         \n  bang-bop: bar;\
         \n}\n"
    );
}
