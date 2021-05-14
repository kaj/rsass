//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn even() {
        assert_eq!(
            runner()
                .ok("a {b: mix(rgba(#91e16f, 0.3), rgba(#0144bf, 0.3))}\n"),
            "a {\
         \n  b: rgba(73, 147, 151, 0.3);\
         \n}\n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            runner().ok("a {b: mix(#91e16f, transparent)}\n"),
            "a {\
         \n  b: rgba(145, 225, 111, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn firstwards() {
        assert_eq!(
            runner()
                .ok("a {b: mix(rgba(#91e16f, 0.8), rgba(#0144bf, 0.3))}\n"),
            "a {\
         \n  b: rgba(109, 186, 131, 0.55);\
         \n}\n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            runner().ok("a {b: mix(transparent, #0144bf)}\n"),
            "a {\
         \n  b: rgba(1, 68, 191, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn lastwards() {
        assert_eq!(
            runner()
                .ok("a {b: mix(rgba(#91e16f, 0.4), rgba(#0144bf, 0.9))}\n"),
            "a {\
         \n  b: rgba(37, 107, 171, 0.65);\
         \n}\n"
        );
    }
}
mod both_weights {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn contradiction() {
        assert_eq!(
        runner().ok(
            "// When we weight entirely towards a transparent color, the formula for\
             \n// computing the combined alpha would divide by zero, so we just return\
             \n// transparent as a special case.\
             \na {b: mix(transparent, #0144bf, 100%)}\n"
        ),
        "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
    );
    }
    mod mixed {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn firstwards() {
            assert_eq!(
        runner().ok(
            "a {b: mix(rgba(#91e16f, 0.8), rgba(#0144bf, 0.3), 63%)}\n"
        ),
        "a {\
         \n  b: rgba(121, 199, 124, 0.615);\
         \n}\n"
    );
        }
        #[test]
        fn lastwards() {
            assert_eq!(
        runner().ok(
            "a {b: mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 42%)}\n"
        ),
        "a {\
         \n  b: rgba(29, 99, 175, 0.49);\
         \n}\n"
    );
        }
    }
    mod transparent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("a {b: mix(transparent, #0144bf, 70%)}\n"),
                "a {\
         \n  b: rgba(1, 68, 191, 0.3);\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("a {b: mix(#91e16f, transparent, 70%)}\n"),
                "a {\
         \n  b: rgba(145, 225, 111, 0.7);\
         \n}\n"
            );
        }
    }
    mod weighted {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
        runner().ok(
            "a {b: mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 100%)}\n"
        ),
        "a {\
         \n  b: rgba(145, 225, 111, 0.2);\
         \n}\n"
    );
        }
        #[test]
        fn last() {
            assert_eq!(
        runner().ok(
            "a {b: mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 0%)}\n"
        ),
        "a {\
         \n  b: rgba(1, 68, 191, 0.7);\
         \n}\n"
    );
        }
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    mod bounds {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn too_high() {
            assert_eq!(
                runner().err("a {b: mix(red, blue, 100.001)}\n"),
                "Error: $weight: Expected 100.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: mix(red, blue, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_low() {
            assert_eq!(
                runner().err("a {b: mix(red, blue, -0.001)}\n"),
                "Error: $weight: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: mix(red, blue, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: mix(red)}\n"),
            "Error: Missing argument $color2.\
         \n  ,--> input.scss\
         \n1 | a {b: mix(red)}\
         \n  |       ^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function mix($color1, $color2, $weight: 50%) {\
         \n  |           =================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: mix(red, blue, 100, 1)}\n"),
            "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: mix(red, blue, 100, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function mix($color1, $color2, $weight: 50%) {\
         \n  |           =================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn color1() {
            assert_eq!(
                runner().err("a {b: mix(1, blue)}\n"),
                "Error: $color1: 1 is not a color.\
         \n  ,\
         \n1 | a {b: mix(1, blue)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn color2() {
            assert_eq!(
                runner().err("a {b: mix(red, 1)}\n"),
                "Error: $color2: 1 is not a color.\
         \n  ,\
         \n1 | a {b: mix(red, 1)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn weight() {
            assert_eq!(
                runner().err("a {b: mix(red, blue, green)}\n"),
                "Error: $weight: green is not a number.\
         \n  ,\
         \n1 | a {b: mix(red, blue, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod explicit_weight {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn even() {
        assert_eq!(
            runner().ok("a {b: mix(#91e16f, #0144bf, 50%)}\n"),
            "a {\
         \n  b: #499397;\
         \n}\n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            runner().ok("a {b: mix(#91e16f, #0144bf, 100%)}\n"),
            "a {\
         \n  b: #91e16f;\
         \n}\n"
        );
    }
    #[test]
    fn firstwards() {
        assert_eq!(
            runner().ok("a {b: mix(#91e16f, #0144bf, 92%)}\n"),
            "a {\
         \n  b: #85d475;\
         \n}\n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            runner().ok("a {b: mix(#91e16f, #0144bf, 0%)}\n"),
            "a {\
         \n  b: #0144bf;\
         \n}\n"
        );
    }
    #[test]
    fn lastwards() {
        assert_eq!(
            runner().ok("a {b: mix(#91e16f, #0144bf, 43%)}\n"),
            "a {\
         \n  b: #3f889d;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: mix($color1: #91e16f, $color2: #0144bf, $weight: 92%)}\n"
        ),
        "a {\
         \n  b: #85d475;\
         \n}\n"
    );
}
mod unweighted {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn average() {
        assert_eq!(
            runner().ok(
                "// All channels should be averaged across the two colors.\
             \na {b: mix(#91e16f, #0144bf)}\n"
            ),
            "a {\
         \n  b: #499397;\
         \n}\n"
        );
    }
    #[test]
    fn identical() {
        assert_eq!(
        runner().ok(
            "// If two channels have the same values, they should be the same in the output.\
             \na {b: mix(#123456, #123456)}\n"
        ),
        "a {\
         \n  b: #123456;\
         \n}\n"
    );
    }
    #[test]
    fn min_and_max() {
        assert_eq!(
        runner().ok(
            "// Each channel becomes the average of 255 and 0, which is 128 = 0xAA.\
             \na {b: mix(#ff00ff, #00ff00)}\n"
        ),
        "a {\
         \n  b: gray;\
         \n}\n"
    );
    }
}
