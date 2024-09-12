//! Tests auto-converted from "sass-spec/spec/core_functions/color/grayscale.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("grayscale")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.grayscale()}\n"
            ),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.grayscale()}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function grayscale($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.grayscale(red, 1)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.grayscale(red, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function grayscale($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.grayscale(c)}\n"
            ),
            "Error: $color: c is not a color.\
         \n  ,\
         \n2 | a {b: color.grayscale(c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod with_module {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn test_type() {
            assert_eq!(
                runner().err(
                    "@use \"sass:color\";\
             \na {b: color.grayscale(var(--c))}\n"
                ),
                "Error: $color: var(--c) is not a color.\
         \n  ,\
         \n2 | a {b: color.grayscale(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
mod global {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn number() {
        assert_eq!(
        runner().ok(
            "// A number should produce a plain function string, for CSS filter functions.\
             \na {b: grayscale(15%)}\n"
        ),
        "a {\
         \n  b: grayscale(15%);\
         \n}\n"
    );
    }
    #[test]
    fn with_calc() {
        assert_eq!(
            runner().ok("a {b: grayscale(calc(1 + 2))}\n"),
            "a {\
         \n  b: grayscale(3);\
         \n}\n"
        );
    }
    #[test]
    fn with_css_var() {
        assert_eq!(
            runner().ok("a {b: grayscale(var(--c))}\n"),
            "a {\
         \n  b: grayscale(var(--c));\
         \n}\n"
        );
    }
    #[test]
    fn with_unquoted_calc() {
        assert_eq!(
            runner().ok("a {b: grayscale(unquote(\'calc(1)\'))}\n"),
            "a {\
         \n  b: grayscale(calc(1));\
         \n}\n"
        );
    }
}
mod legacy {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn alpha() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(rgba(#633736, 0.3))}\n"),
            "a {\
         \n  b: rgba(76.5, 76.5, 76.5, 0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max_saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(red)}\n"),
            "a {\
         \n  b: rgb(127.5, 127.5, 127.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn mid_saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(#633736)}\n"),
            "a {\
         \n  b: rgb(76.5, 76.5, 76.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn missing() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(hsl(none none none))}\n"),
            "a {\
         \n  b: hsl(none 0% none);\
         \n}\n"
        );
    }
    mod no_saturation {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn black() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(black)}\n"),
                "a {\
         \n  b: black;\
         \n}\n"
            );
        }
        #[test]
        fn gray() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(#494949)}\n"),
                "a {\
         \n  b: #494949;\
         \n}\n"
            );
        }
        #[test]
        fn white() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(white)}\n"),
                "a {\
         \n  b: white;\
         \n}\n"
            );
        }
    }
    mod powerless {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(hwb(120deg 10% 20%))}\n"),
                "a {\
         \n  b: hsl(0, 0%, 45%);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(hsl(120deg 10% 20%))}\n"),
                "a {\
         \n  b: hsl(120, 0%, 20%);\
         \n}\n"
            );
        }
    }
}
mod modern {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn missing() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(oklch(none none none))}\n"),
            "a {\
         \n  b: oklch(none 0 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn polar() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(lch(54.3 107 40.9))}\n"),
            "a {\
         \n  b: lch(56.854581217% 0.0000089094 270.4699001175deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn powerless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(oklch(20% 10% 120deg))}\n"),
            "a {\
         \n  b: oklch(20% 0 120deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn predefined() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(color(xyz-d65 0.41 0.21 0.02))}\n"),
            "a {\
         \n  b: color(xyz 0.232829773 0.2449664044 0.2667826176);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn rectangular() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(lab(50 -30 40))}\n"),
            "a {\
         \n  b: lab(48.58892555% 0.0000000648 -0.0000078984);\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale($color: white)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
#[test]
fn number() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(1)}\n"),
        "a {\
         \n  b: grayscale(1);\
         \n}\n"
    );
}
