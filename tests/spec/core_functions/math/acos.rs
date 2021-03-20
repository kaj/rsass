//! Tests auto-converted from "sass-spec/spec/core_functions/math/acos.hrx"

#[test]
fn decimal() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 60deg;\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.

    // Ignoring "units", error tests are not supported yet.

    // Ignoring "zero_args", error tests are not supported yet.
}
#[test]
fn greater_than_one() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: NaNdeg;\
        \n}\
        \n"
    );
}
#[test]
fn less_than_negative_one() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(-2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: NaNdeg;\
        \n}\
        \n"
    );
}
#[test]
fn negative_decimal() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(-0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 120deg;\
        \n}\
        \n"
    );
}
#[test]
fn one() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0deg;\
        \n}\
        \n"
    );
}
#[test]
fn one_fuzzy() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(1.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0deg;\
        \n}\
        \n"
    );
}
