//! Tests auto-converted from "sass-spec/spec/core_functions/color/invert/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod bounds {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.invert(red, 100.001%)}\n"
            ),
            "Error: $weight: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.invert(red, 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.invert(red, -0.001%)}\n"
            ),
            "Error: $weight: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n2 | a {b: color.invert(red, -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod global {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn number_with_weight() {
        assert_eq!(
        runner().err(
            "a {b: invert(1, 50%)}\n"
        ),
        "Error: Only one argument may be passed to the plain-CSS invert() function.\
         \n  ,\
         \n1 | a {b: invert(1, 50%)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn color() {
            assert_eq!(
        runner().err(
            "a {b: invert(c)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.invert instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: invert(c)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $color: c is not a color.\
         \n  ,\
         \n1 | a {b: invert(c)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
}
mod missing {
    #[allow(unused)]
    use super::runner;

    mod legacy {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn analogous() {
            assert_eq!(
        runner().err(
            "a {b: invert(rgb(10 none 20), $space: xyz)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.invert instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: invert(rgb(10 none 20), $space: xyz)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $y: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: color(xyz 0.0025142545 none 0.0067080366)).\
         \n  ,\
         \n1 | a {b: invert(rgb(10 none 20), $space: xyz)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn powerless() {
            assert_eq!(
        runner().err(
            "a {b: invert(grey, $space: hsl)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.invert instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: invert(grey, $space: hsl)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: hsl(none 0% 50.1960784314%)).\
         \n  ,\
         \n1 | a {b: invert(grey, $space: hsl)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        mod same {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong error
            fn explicit() {
                assert_eq!(
        runner().err(
            "a {b: invert(hsl(0 40% none), $space: hsl)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.invert instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: invert(hsl(0 40% none), $space: hsl)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $lightness: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: hsl(0deg 40% none)).\
         \n  ,\
         \n1 | a {b: invert(hsl(0 40% none), $space: hsl)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn hwb_hue() {
                assert_eq!(
        runner().err(
            "a {b: invert(hwb(none 10% 20%), $space: hwb)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.invert instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: invert(hwb(none 10% 20%), $space: hwb)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: hwb(none 10% 20%)).\
         \n  ,\
         \n1 | a {b: invert(hwb(none 10% 20%), $space: hwb)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn implicit() {
                assert_eq!(
        runner().err(
            "a {b: invert(rgb(none 10 20))}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.invert instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: invert(rgb(none 10 20))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $red: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: rgb(none 10 20)).\
         \n  ,\
         \n1 | a {b: invert(rgb(none 10 20))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
            }
        }
    }
    mod modern {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn analogous() {
            assert_eq!(
        runner().err(
            "a {b: invert(color(rec2020 0.1 none 0.2), $space: xyz)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.invert instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: invert(color(rec2020 0.1 none 0.2), $space: xyz)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $y: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: color(xyz 0.0237000113 none 0.0589013339)).\
         \n  ,\
         \n1 | a {b: invert(color(rec2020 0.1 none 0.2), $space: xyz)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn powerless() {
            assert_eq!(
        runner().err(
            "a {b: invert(color(rec2020 0.4 0.4 0.4), $space: lch)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.invert instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: invert(color(rec2020 0.4 0.4 0.4), $space: lch)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: lch(48.649404846% 0 none)).\
         \n  ,\
         \n1 | a {b: invert(color(rec2020 0.4 0.4 0.4), $space: lch)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn same() {
            assert_eq!(
        runner().err(
            "a {b: invert(color(srgb none 0.1 0.2), $space: srgb)}\n"
        ),
        "DEPRECATION WARNING: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.invert instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: invert(color(srgb none 0.1 0.2), $space: srgb)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $red: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: color(srgb none 0.1 0.2)).\
         \n  ,\
         \n1 | a {b: invert(color(srgb none 0.1 0.2), $space: srgb)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
}
mod space {
    #[allow(unused)]
    use super::runner;

    mod missing {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn default_weight() {
            assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.invert(lab(50% -10 10))}\n"
        ),
        "Error: $color: To use color.invert() with non-legacy color lab(50% -10 10), you must provide a $space.\
         \n  ,\
         \n2 | a {b: color.invert(lab(50% -10 10))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn zero_weight() {
            assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.invert(lab(50% -10 10), 0%)}\n"
        ),
        "Error: $color: To use color.invert() with non-legacy color lab(50% -10 10), you must provide a $space.\
         \n  ,\
         \n2 | a {b: color.invert(lab(50% -10 10), 0%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn quoted() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.invert(red, 10%, \"lch\")}\n"
            ),
            "Error: $space: Expected \"lch\" to be an unquoted string.\
         \n  ,\
         \n2 | a {b: color.invert(red, 10%, \"lch\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn unknown() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.invert(red, 10%, c)}\n"
            ),
            "Error: $space: Unknown color space \"c\".\
         \n  ,\
         \n2 | a {b: color.invert(red, 10%, c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn too_few_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.invert()}\n"
        ),
        "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.invert()}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function invert($color, $weight: 100%, $space: null) {\
         \n  |           =========================================== declaration\
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
             \na {b: color.invert(turquoise, 0%, hsl, 1)}\n"
        ),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.invert(turquoise, 0%, hsl, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function invert($color, $weight: 100%, $space: null) {\
         \n  |           =========================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod test_type {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn color() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.invert(c)}\n"
            ),
            "Error: $color: c is not a color.\
         \n  ,\
         \n2 | a {b: color.invert(c)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn space() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.invert(red, 10%, 1)}\n"
            ),
            "Error: $space: 1 is not a string.\
         \n  ,\
         \n2 | a {b: color.invert(red, 10%, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn weight() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.invert(red, c)}\n"
            ),
            "Error: $weight: c is not a number.\
         \n  ,\
         \n2 | a {b: color.invert(red, c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
