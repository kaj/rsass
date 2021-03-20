//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix.hrx"

mod alpha {
    #[test]
    fn even() {
        assert_eq!(
            crate::rsass(
                "a {b: mix(rgba(#91e16f, 0.3), rgba(#0144bf, 0.3))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(73, 147, 151, 0.3);\
        \n}\
        \n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            crate::rsass(
                "a {b: mix(#91e16f, transparent)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(145, 225, 111, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn firstwards() {
        assert_eq!(
            crate::rsass(
                "a {b: mix(rgba(#91e16f, 0.8), rgba(#0144bf, 0.3))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(109, 186, 131, 0.55);\
        \n}\
        \n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            crate::rsass(
                "a {b: mix(transparent, #0144bf)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(1, 68, 191, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn lastwards() {
        assert_eq!(
            crate::rsass(
                "a {b: mix(rgba(#91e16f, 0.4), rgba(#0144bf, 0.9))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(37, 107, 171, 0.65);\
        \n}\
        \n"
        );
    }
}
mod both_weights {
    #[test]
    fn contradiction() {
        assert_eq!(
        crate::rsass(
            "// When we weight entirely towards a transparent color, the formula for\
            \n// computing the combined alpha would divide by zero, so we just return\
            \n// transparent as a special case.\
            \na {b: mix(transparent, #0144bf, 100%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(0, 0, 0, 0);\
        \n}\
        \n"
    );
    }
    mod mixed {
        #[test]
        fn firstwards() {
            assert_eq!(
                crate::rsass(
                    "a {b: mix(rgba(#91e16f, 0.8), rgba(#0144bf, 0.3), 63%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(121, 199, 124, 0.615);\
        \n}\
        \n"
            );
        }
        #[test]
        fn lastwards() {
            assert_eq!(
                crate::rsass(
                    "a {b: mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 42%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(29, 99, 175, 0.49);\
        \n}\
        \n"
            );
        }
    }
    mod transparent {
        #[test]
        fn first() {
            assert_eq!(
                crate::rsass(
                    "a {b: mix(transparent, #0144bf, 70%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 68, 191, 0.3);\
        \n}\
        \n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                crate::rsass(
                    "a {b: mix(#91e16f, transparent, 70%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(145, 225, 111, 0.7);\
        \n}\
        \n"
            );
        }
    }
    mod weighted {
        #[test]
        fn first() {
            assert_eq!(
        crate::rsass(
            "a {b: mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 100%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(145, 225, 111, 0.2);\
        \n}\
        \n"
    );
        }
        #[test]
        fn last() {
            assert_eq!(
                crate::rsass(
                    "a {b: mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 0%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 68, 191, 0.7);\
        \n}\
        \n"
            );
        }
    }
}
mod error {
    mod bounds {

        // Ignoring "too_high", error tests are not supported yet.

        // Ignoring "too_low", error tests are not supported yet.
    }

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "color1", error tests are not supported yet.

        // Ignoring "color2", error tests are not supported yet.

        // Ignoring "weight", error tests are not supported yet.
    }
}
mod explicit_weight {
    #[test]
    fn even() {
        assert_eq!(
            crate::rsass(
                "a {b: mix(#91e16f, #0144bf, 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #499397;\
        \n}\
        \n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            crate::rsass(
                "a {b: mix(#91e16f, #0144bf, 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #91e16f;\
        \n}\
        \n"
        );
    }
    #[test]
    fn firstwards() {
        assert_eq!(
            crate::rsass(
                "a {b: mix(#91e16f, #0144bf, 92%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #85d475;\
        \n}\
        \n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            crate::rsass(
                "a {b: mix(#91e16f, #0144bf, 0%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #0144bf;\
        \n}\
        \n"
        );
    }
    #[test]
    fn lastwards() {
        assert_eq!(
            crate::rsass(
                "a {b: mix(#91e16f, #0144bf, 43%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #3f889d;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: mix($color1: #91e16f, $color2: #0144bf, $weight: 92%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #85d475;\
        \n}\
        \n"
    );
}
mod unweighted {
    #[test]
    fn average() {
        assert_eq!(
            crate::rsass(
                "// All channels should be averaged across the two colors.\
            \na {b: mix(#91e16f, #0144bf)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #499397;\
        \n}\
        \n"
        );
    }
    #[test]
    fn identical() {
        assert_eq!(
        crate::rsass(
            "// If two channels have the same values, they should be the same in the output.\
            \na {b: mix(#123456, #123456)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #123456;\
        \n}\
        \n"
    );
    }
    #[test]
    fn min_and_max() {
        assert_eq!(
        crate::rsass(
            "// Each channel becomes the average of 255 and 0, which is 128 = 0xAA.\
            \na {b: mix(#ff00ff, #00ff00)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: gray;\
        \n}\
        \n"
    );
    }
}
