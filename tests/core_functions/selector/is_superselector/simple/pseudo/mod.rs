//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/arg.hrx"
mod arg {
    #[allow(unused)]
    use super::rsass;
    mod class {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn equal() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":c(@#$)\", \":c(@#$)\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: true;\
        \n}\
        \n"
            );
        }
        mod unequal {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn argument() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\":c(@#$)\", \":c(*&^)\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: false;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn has_argument() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\":c(@#$)\", \":c\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: false;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn name() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\":c(@#$)\", \":d(@#$)\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: false;\
        \n}\
        \n"
                );
            }
        }
    }
    mod element {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn equal() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"::c(@#$)\", \"::c(@#$)\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: true;\
        \n}\
        \n"
            );
        }
        mod unequal {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn argument() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"::c(@#$)\", \"::c(*&^)\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: false;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn has_argument() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"::c(@#$)\", \"::c\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: false;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn name() {
                assert_eq!(
                    rsass(
                        "a {b: is-superselector(\"::c(@#$)\", \":d(@#$)\")}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: false;\
        \n}\
        \n"
                );
            }
        }
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/no_arg.hrx"
mod no_arg {
    #[allow(unused)]
    use super::rsass;
    mod class {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn and_element() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":c\", \"::c\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: false;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn equal() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":c\", \":c\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn unequal() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":c\", \":d\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: false;\
        \n}\
        \n"
            );
        }
    }
    mod element {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn and_class() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"::c\", \":c\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: false;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn equal() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"::c\", \"::c\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn unequal() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\"::c\", \"::d\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: false;\
        \n}\
        \n"
            );
        }
    }
}

mod selector_arg;
