//! Tests auto-converted from "sass-spec/spec/core_functions/math/min.hrx"

mod error {

    // Ignoring "incompatible_units", error tests are not supported yet.

    // Ignoring "too_few_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "arg_1", error tests are not supported yet.

        // Ignoring "arg_2", error tests are not supported yet.

        // Ignoring "arg_3", error tests are not supported yet.
    }
}
#[test]
fn one_arg() {
    assert_eq!(
        crate::rsass(
            "$arg: 1;\
            \na {b: min($arg)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn three_args() {
    assert_eq!(
        crate::rsass(
            "$arg: 1;\
            \na {b: min(3, $arg, 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn two_args() {
    assert_eq!(
        crate::rsass(
            "$arg: 1;\
            \na {b: min($arg, 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
mod units {
    #[test]
    fn and_unitless() {
        assert_eq!(
            crate::rsass(
                "$arg: 2px;\
            \na {b: min($arg, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn compatible() {
        assert_eq!(
            crate::rsass(
                "$arg: 1px;\
            \na {b: min($arg, 1in, 1cm)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1px;\
        \n}\
        \n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            crate::rsass(
                "$arg: 6px;\
            \na {b: min($arg, 2px, 10px)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2px;\
        \n}\
        \n"
        );
    }
}
