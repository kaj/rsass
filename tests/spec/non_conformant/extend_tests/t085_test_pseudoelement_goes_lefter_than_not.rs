//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/085_test_pseudoelement_goes_lefter_than_not.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo::bar {a: b}\
             \n.baz:not(.bang) {@extend .foo}\n"),
        ".foo::bar, .baz:not(.bang)::bar {\
         \n  a: b;\
         \n}\n"
    );
}
