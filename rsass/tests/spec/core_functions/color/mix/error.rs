//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod bounds {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn too_high() {
        assert_eq!(
        runner().err(
            "a {b: mix(red, blue, 100.001%)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $weight: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: mix(red, blue, 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn too_low() {
        assert_eq!(
        runner().err(
            "a {b: mix(red, blue, -0.001%)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $weight: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: mix(red, blue, -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong error
fn color_interpolation_method() {
    assert_eq!(
        runner().err(
            "a {b: mix(red, blue, $method: brighter)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: brighter)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $method: Unknown color space \"brighter\".\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: brighter)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn extra_character_end() {
    assert_eq!(
        runner().err("a {b: mix(red, blue, $method: hsl longer hue.)}\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: hsl longer hue.)}\
         \n  |                                              ^\
         \n  \'\
         \n  input.scss 1:46  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn extra_character_start() {
    assert_eq!(
        runner().err("a {b: mix(red, blue, $method: ,hsl longer hue)}\n"),
        "Error: Expected expression.\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: ,hsl longer hue)}\
         \n  |                               ^\
         \n  \'\
         \n  input.scss 1:31  root stylesheet",
    );
}
mod interpolation {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn number() {
        assert_eq!(
        runner().err(
            "a {b: mix(red, blue, $method: hsl 1)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: hsl 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $method: 1 is not a string.\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: hsl 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
mod interpolation_list {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn quoted() {
        assert_eq!(
        runner().err(
            "a {b: mix(red, blue, $method: \"hsl longer hue\")}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: \"hsl longer hue\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $method: Expected \"hsl longer hue\" to be an unquoted string.\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: \"hsl longer hue\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn separator() {
        assert_eq!(
            runner().err("a {b: mix(red, blue, $method: hsl, longer hue)}\n"),
            "Error: Positional arguments must come before keyword arguments.\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: hsl, longer hue)}\
         \n  |                                    ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:36  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn slash() {
        assert_eq!(
        runner().err(
            "a {b: mix(red, blue, $method: hsl/longer/hue)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: hsl/longer/hue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $method: Unknown color space \"hsl/longer/hue\".\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: hsl/longer/hue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
mod invalid_method {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn unsupported() {
        assert_eq!(
        runner().err(
            "// \"specified\" was a method in an earlier draft of CSS Colors 4, but it was\
             \n// dropped because it required implementations to lazily normalize hues.\
             \na {b: mix(red, blue, $method: hsl specified hue)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n3 | a {b: mix(red, blue, $method: hsl specified hue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 3:7  root stylesheet\n\
         \nError: $method: Unknown hue interpolation method specified.\
         \n  ,\
         \n3 | a {b: mix(red, blue, $method: hsl specified hue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn wrong_hue_keyword() {
        assert_eq!(
        runner().err(
            "a {b: mix(red, blue, $method: hsl longer shade)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: hsl longer shade)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $method: Expected unquoted string \"hue\" at the end of (hsl longer shade), was shade.\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: hsl longer shade)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong error
fn missing_hue_string() {
    assert_eq!(
        runner().err(
            "a {b: mix(red, blue, $method: lch decreasing)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: lch decreasing)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $method: Expected unquoted string \"hue\" after (lch decreasing).\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: lch decreasing)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
mod null_method {
    #[allow(unused)]
    use super::runner;

    mod non_legacy {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn color1() {
            assert_eq!(
        runner().err(
            "a {b: mix(lch(20% -20 0), red)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(lch(20% -20 0), red)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $color1: To use color.mix() with non-legacy color lch(20% 0 0deg), you must provide a $method.\
         \n  ,\
         \n1 | a {b: mix(lch(20% -20 0), red)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn color2() {
            assert_eq!(
        runner().err(
            "a {b: mix(red, lch(20% -20 0))}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, lch(20% -20 0))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $color2: To use color.mix() with non-legacy color lch(20% 0 0deg), you must provide a $method.\
         \n  ,\
         \n1 | a {b: mix(red, lch(20% -20 0))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
}
#[test]
#[ignore] // wrong error
fn parentheses() {
    assert_eq!(
        runner().err(
            "a {b: mix(red, blue, $method: lch (decreasing hue))}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: lch (decreasing hue))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $method: (decreasing hue) is not a string.\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: lch (decreasing hue))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn rectangular_space_with_method() {
    assert_eq!(
        runner().err(
            "a {b: mix(red, blue, $method: srgb longer hue)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: srgb longer hue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $method: Hue interpolation method \"HueInterpolationMethod.longer hue\" may not be set for rectangular color space srgb.\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: srgb longer hue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn too_few_args() {
    assert_eq!(
        runner().err(
            "a {b: mix(red)}\n"
        ),
        "Error: Missing argument $color2.\
         \n  ,--> input.scss\
         \n1 | a {b: mix(red)}\
         \n  |       ^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function mix($color1, $color2, $weight: 50%, $method: null) {\
         \n  |           ================================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn too_many_args() {
    assert_eq!(
        runner().err(
            "a {b: mix(red, blue, 30%, lch, 1)}\n"
        ),
        "Error: Only 4 arguments allowed, but 5 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: mix(red, blue, 30%, lch, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function mix($color1, $color2, $weight: 50%, $method: null) {\
         \n  |           ================================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
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
            "a {b: mix(1, blue)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(1, blue)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $color1: 1 is not a color.\
         \n  ,\
         \n1 | a {b: mix(1, blue)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn color2() {
        assert_eq!(
        runner().err(
            "a {b: mix(red, 1)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, 1)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $color2: 1 is not a color.\
         \n  ,\
         \n1 | a {b: mix(red, 1)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn weight() {
        assert_eq!(
        runner().err(
            "a {b: mix(red, blue, green)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $weight: green is not a number.\
         \n  ,\
         \n1 | a {b: mix(red, blue, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong error
fn unknown_interpolation() {
    assert_eq!(
        runner().err(
            "a {b: mix(red, blue, $method: lch longerhue)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.mix instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: lch longerhue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $method: Unknown hue interpolation method longerhue.\
         \n  ,\
         \n1 | a {b: mix(red, blue, $method: lch longerhue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
