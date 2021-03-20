//! Tests auto-converted from "sass-spec/spec/core_functions/color/desaturate.hrx"

#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: desaturate(rgba(plum, 0.3), 100%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(191, 191, 191, 0.3);\
        \n}\
        \n"
    );
}
mod error {
    mod bounds {

        // Ignoring "too_high", error tests are not supported yet.

        // Ignoring "too_low", error tests are not supported yet.
    }
    mod one_arg {

        // Ignoring "test_type", error tests are not supported yet.
    }

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "color", error tests are not supported yet.

        // Ignoring "lightness", error tests are not supported yet.
    }
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: desaturate(plum, 100%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #bfbfbf;\
        \n}\
        \n"
    );
}
#[test]
fn max_remaining() {
    assert_eq!(
        crate::rsass(
            "a {b: desaturate(plum, 48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #bfbfbf;\
        \n}\
        \n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: desaturate(plum, 14%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #d4a9d4;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: desaturate(plum, 0%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: plum;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: desaturate($color: plum, $amount: 14%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #d4a9d4;\
        \n}\
        \n"
    );
}
