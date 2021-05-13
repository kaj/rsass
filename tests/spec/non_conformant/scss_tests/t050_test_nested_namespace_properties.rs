//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/050_test_nested_namespace_properties.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
