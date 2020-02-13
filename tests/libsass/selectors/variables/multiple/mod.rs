//! Tests auto-converted from "sass-spec/spec/libsass/selectors/variables/multiple"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/libsass/selectors/variables/multiple/bare.hrx"
#[test]
#[ignore] // wrong result
fn bare() {
    assert_eq!(
        rsass(
            ".foo a,\
            \n.bar p {\
            \n  $bar: &;\
            \n  content: $bar;\
            \n}"
        )
        .unwrap(),
        ".foo a,\
        \n.bar p {\
        \n  content: .foo a, .bar p;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/selectors/variables/multiple/interpolated.hrx"
#[test]
#[ignore] // wrong result
fn interpolated() {
    assert_eq!(
        rsass(
            ".foo a,\
            \n.bar p {\
            \n  $bar: &;\
            \n  content: #{$bar};\
            \n}"
        )
        .unwrap(),
        ".foo a,\
        \n.bar p {\
        \n  content: .foo a, .bar p;\
        \n}\
        \n"
    );
}
