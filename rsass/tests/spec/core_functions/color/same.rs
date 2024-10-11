//! Tests auto-converted from "sass-spec/spec/core_functions/color/same.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("same")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.same(plum)}\n"
            ),
            "Error: Missing argument $color2.\
         \n  ,--> input.scss\
         \n2 | a {b: color.same(plum)}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function same($color1, $color2) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.same(red, green, blue)}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.same(red, green, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function same($color1, $color2) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn color1() {
            assert_eq!(
                runner().err(
                    "@use \"sass:color\";\
             \na {b: color.same(1, red)}\n"
                ),
                "Error: $color1: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.same(1, red)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn color2() {
            assert_eq!(
                runner().err(
                    "@use \"sass:color\";\
             \na {b: color.same(red, 1)}\n"
                ),
                "Error: $color2: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.same(red, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
mod test_false {
    #[allow(unused)]
    use super::runner;

    mod different_space {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn both_none() {
            assert_eq!(
        runner().ok(
            "// This test verifies that none is converted to 0 *before* conversion to XYZ.\
             \n@use \"sass:color\";\
             \na {\
             \n  b: color.same(\
             \n    color(rec2020 0.5 none 0.2),\
             \n    color(xyz 0.174805932224126 none 0.058901333881161)\
             \n  );\
             \n}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn no_none() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.same(color(srgb 0.1 0.2 0.3), color(srgb-linear 0.1 0.2 0.3))}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
        }
    }
    mod same_space {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn no_none() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.same(#abcdef, #fedcba)}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn none_and_nonzero() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.same(\
             \n    color(prophoto-rgb 0.1 0.2 none),\
             \n    color(prophoto-rgb 0.1 0.2 0.3)\
             \n  );\
             \n}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.same($color1: red, $color2: green)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod test_true {
    #[allow(unused)]
    use super::runner;

    mod different_space {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn both_none() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.same(\
             \n    color(srgb-linear none 0.9 0.8),\
             \n    rgb(none 243.445228830895 231.114597102735)\
             \n  );\
             \n}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn no_none() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.same(plum, hsl(300, 47.2868217054%, 74.7058823529%))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn one_none() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.same(\
             \n    color(rec2020 0.5 none 0.2),\
             \n    oklab(44.66886691637825% 0.2366736512579 0.01872833430856)\
             \n  );\
             \n}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
    mod same_space {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn both_none() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.same(color(display-p3 0.1 0.3 none), color(display-p3 0.1 0.3 none));\
             \n}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn identical() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.same(#abcdef, #abcdef)}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn none_and_zero() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.same(color(display-p3 0.1 0.3 none), color(display-p3 0.1 0.3 0))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn out_of_gamut() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.same(color(srgb 2.3 -1 42), color(srgb 2.3 -1 42))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn with_alpha() {
            assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.same(oklch(50% 30% 120deg / 0.3), oklch(50% 30% 120deg / 0.3))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn within_epsilon() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.same(\
             \n    lab(50.0000000000001 29.9999999999999 -20.0000000000001),\
             \n    lab(50 30 -20)\
             \n  );\
             \n}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
}
