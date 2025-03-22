//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/nested_namespace_properties.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested_namespace_properties")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: baz;\
             \n  bang: {\
             \n    bip: 1px;\
             \n    bop: bar;\
             \n    blat:{baf:bort}}}\n"),
        "foo {\
         \n  bar: baz;\
         \n  bang-bip: 1px;\
         \n  bang-bop: bar;\
         \n  bang-blat-baf: bort;\
         \n}\n"
    );
}
