//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/086_test_pseudoelement_goes_lefter_than_not.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo:not(.bang) {a: b}\
            \n.baz::bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo:not(.bang), .baz:not(.bang)::bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
