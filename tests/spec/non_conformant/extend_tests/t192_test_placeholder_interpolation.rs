//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/192_test_placeholder_interpolation.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: foo;\
            \n\
            \n%#{$foo} {color: blue}\
            \n.bar {@extend %foo}\
            \n"
        )
        .unwrap(),
        ".bar {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
