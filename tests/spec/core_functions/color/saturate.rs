//! Tests auto-converted from "sass-spec/spec/core_functions/color/saturate.hrx"

mod error {
    mod one_arg {

        // Ignoring "test_type", error tests are not supported yet.
    }

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod two_args {
        mod bounds {

            // Ignoring "too_high", error tests are not supported yet.

            // Ignoring "too_low", error tests are not supported yet.
        }
        mod test_type {

            // Ignoring "color", error tests are not supported yet.

            // Ignoring "lightness", error tests are not supported yet.
        }
    }
}
mod one_arg {
    #[test]
    fn named() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate($amount: 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: saturate(50%);\
        \n}\
        \n"
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: saturate(50%);\
        \n}\
        \n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: saturate(1);\
        \n}\
        \n"
        );
    }
}
mod two_args {
    #[test]
    fn alpha() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(rgba(plum, 0.5), 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 126, 255, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(plum, 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff7eff;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max_remaining() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(plum, 53%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff7eff;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(plum, 14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #e697e6;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(plum, 0%)}\
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
                "a {b: saturate($color: plum, $amount: 14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #e697e6;\
        \n}\
        \n"
        );
    }
}
