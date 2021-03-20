//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/085_test_pseudoelement_goes_lefter_than_not.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo::bar {a: b}\
            \n.baz:not(.bang) {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo::bar, .baz:not(.bang)::bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
