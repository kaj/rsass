//! Tests auto-converted from "sass-spec/spec/libsass/units"
#[allow(unused)]
use super::rsass;

mod conversion;

// From "sass-spec/spec/libsass/units/feature-test.hrx"
#[test]
fn feature_test() {
    assert_eq!(
        rsass(
            "@if feature-exists(units-level-3) {\
            \n  div {\
            \n    feature: true;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  feature: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/units/simple.hrx"
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        rsass(
            "div {\
            \n  hey: ((5in + 3cm) * 10px * 100pt * 10fu / 2px / 2fu / 3pt);\
            \n  ho: (23in/2fu) > (23cm/2fu);\
            \n  hoo: unit((23px/2fu/12emu/1.2gnu));\
            \n  hee: unit((2in/3cm/4cm));\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  hey: 370866.1417322835pt;\
        \n  ho: true;\
        \n  hoo: \"px/fu*emu*gnu\";\
        \n  hee: \"cm^-1\";\
        \n}\
        \n"
    );
}
