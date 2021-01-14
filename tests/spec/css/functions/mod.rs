//! Tests auto-converted from "sass-spec/spec/css/functions"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/css/functions/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;
    mod single_equals {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "no_lhs", error tests are not supported yet.

        // Ignoring "no_lhs_or_rhs", error tests are not supported yet.

        // Ignoring "no_rhs", error tests are not supported yet.
    }
}

mod min_max;

// From "sass-spec/spec/css/functions/special.hrx"
mod special {
    #[allow(unused)]
    use super::rsass;
    mod clamp {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn interpolation() {
            assert_eq!(
                rsass(
                    "a {b: clamp(#{0}, #{1}, #{2})}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: clamp(0, 1, 2);\
        \n}\
        \n"
            );
        }
        #[test]
        fn numbers() {
            assert_eq!(
                rsass(
                    "a {b: clamp(0, 1, 2)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: clamp(0, 1, 2);\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn punctuation() {
            assert_eq!(
                rsass(
                    "a {b: clamp(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: clamp(@#$%^&*({[]})_-+=|\\\\:\"\"\"\"<>,.?/);\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/css/functions/url.hrx"
mod url {
    #[allow(unused)]
    use super::rsass;
    mod exclam {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn middle() {
            assert_eq!(
                rsass(
                    "a {b: url(http://c.d/e!f)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: url(http://c.d/e!f);\
        \n}\
        \n"
            );
        }
        #[test]
        fn only() {
            assert_eq!(
                rsass(
                    "a {b: url(!)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: url(!);\
        \n}\
        \n"
            );
        }
    }
}
