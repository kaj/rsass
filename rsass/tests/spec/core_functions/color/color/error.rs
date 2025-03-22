//! Tests auto-converted from "sass-spec/spec/core_functions/color/color/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod list {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn after_space() {
        assert_eq!(
        runner().err(
            "a {b: color(srgb (0.1 0.2 0.3))}\n"
        ),
        "Error: $description: Expected red channel to be a number, was (0.1 0.2 0.3).\
         \n  ,\
         \n1 | a {b: color(srgb (0.1 0.2 0.3))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn bracketed() {
        assert_eq!(
        runner().err(
            "a {b: color([srgb 0.1 0.2 0.3])}\n"
        ),
        "Error: $description: Expected an unbracketed list, was [srgb 0.1 0.2 0.3]\
         \n  ,\
         \n1 | a {b: color([srgb 0.1 0.2 0.3])}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn comma() {
        assert_eq!(
        runner().err(
            "a {b: color((srgb, 0.1, 0.2, 0.3))}\n"
        ),
        "Error: $description: Expected a space- or slash-separated list, was (srgb, 0.1, 0.2, 0.3)\
         \n  ,\
         \n1 | a {b: color((srgb, 0.1, 0.2, 0.3))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    mod slash {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn one() {
            assert_eq!(
        runner().err(
            "@use \'sass:list\';\
             \n$single-arg-slash-separated: list.append((), srgb 0.1 0.2 0.3, $separator: slash);\
             \na {b: color($single-arg-slash-separated)}\n"
        ),
        "Error: $description: Only 2 slash-separated elements allowed, but 1 was passed.\
         \n  ,\
         \n3 | a {b: color($single-arg-slash-separated)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn three() {
            assert_eq!(
        runner().err(
            "@use \'sass:list\';\
             \na {b: color(list.slash(srgb 0.1, 0.2, 0.3))}\n"
        ),
        "Error: $description: Only 2 slash-separated elements allowed, but 3 were passed.\
         \n  ,\
         \n2 | a {b: color(list.slash(srgb 0.1, 0.2, 0.3))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn too_few_channels() {
        assert_eq!(
        runner().err(
            "a {b: color(srgb 0.1 0.2)}\n"
        ),
        "Error: $description: The srgb color space has 3 channels but (srgb 0.1 0.2) has 2.\
         \n  ,\
         \n1 | a {b: color(srgb 0.1 0.2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn too_many_channels() {
        assert_eq!(
        runner().err(
            "a {b: color(srgb 0.1 0.2 0.3 0.4)}\n"
        ),
        "Error: $description: The srgb color space has 3 channels but (srgb 0.1 0.2 0.3 0.4) has 4.\
         \n  ,\
         \n1 | a {b: color(srgb 0.1 0.2 0.3 0.4)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
mod relative_color {
    use super::runner;

    mod quoted {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn alpha() {
            assert_eq!(
        runner().err(
            "a {b: color(\"from\" #aaa srgb r g b / 25%)}\n"
        ),
        "Error: $description: Expected \"from\" to be an unquoted string.\
         \n  ,\
         \n1 | a {b: color(\"from\" #aaa srgb r g b / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn no_alpha() {
            assert_eq!(
        runner().err(
            "a {b: color(\"from\" #aaa srgb r g b)}\n"
        ),
        "Error: $description: Expected \"from\" to be an unquoted string.\
         \n  ,\
         \n1 | a {b: color(\"from\" #aaa srgb r g b)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
    mod wrong_keyword {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn alpha() {
            assert_eq!(
                runner().err("a {b: color(c #aaa srgb r g b / 25%)}\n"),
                "Error: $description: Unknown color space \"c\".\
         \n  ,\
         \n1 | a {b: color(c #aaa srgb r g b / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn no_alpha() {
            assert_eq!(
                runner().err("a {b: color(c #aaa srgb r g b)}\n"),
                "Error: $description: Unknown color space \"c\".\
         \n  ,\
         \n1 | a {b: color(c #aaa srgb r g b)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod too_few_args {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn no_channels() {
        assert_eq!(
        runner().err(
            "a {b: color(srgb)}\n"
        ),
        "Error: $description: The srgb color space has 3 channels but srgb has 0.\
         \n  ,\
         \n1 | a {b: color(srgb)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn no_space() {
        assert_eq!(
            runner().err("a {b: color(1 2 3)}\n"),
            "Error: $description: 1 is not a string.\
         \n  ,\
         \n1 | a {b: color(1 2 3)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn none() {
        assert_eq!(
            runner().err("a {b: color()}\n"),
            "Error: Missing argument $description.\
         \n  ,--> input.scss\
         \n1 | a {b: color()}\
         \n  |       ^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function color($description) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
#[ignore] // missing error
fn too_many_args() {
    assert_eq!(
        runner().err(
            "a {b: color(srgb 0.1 0.2 0.3 0.4)}\n"
        ),
        "Error: $description: The srgb color space has 3 channels but (srgb 0.1 0.2 0.3 0.4) has 4.\
         \n  ,\
         \n1 | a {b: color(srgb 0.1 0.2 0.3 0.4)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
mod test_type {
    use super::runner;

    mod alpha {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn slash_list() {
            assert_eq!(
                runner().err(
                    "@use \'sass:list\';\
             \na {b: color(list.slash(srgb 0.1 0.2 0.3, c))}\n"
                ),
                "Error: $description: c is not a number.\
         \n  ,\
         \n2 | a {b: color(list.slash(srgb 0.1 0.2 0.3, c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn blue() {
        assert_eq!(
        runner().err(
            "a {b: color(srgb 0.1 0.2 c)}\n"
        ),
        "Error: $description: Expected blue channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: color(srgb 0.1 0.2 c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn green() {
        assert_eq!(
        runner().err(
            "a {b: color(srgb 0.1 c 0.3)}\n"
        ),
        "Error: $description: Expected green channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: color(srgb 0.1 c 0.3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn red() {
        assert_eq!(
        runner().err(
            "a {b: color(srgb c 0.2 0.3)}\n"
        ),
        "Error: $description: Expected red channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: color(srgb c 0.2 0.3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
mod unit {
    use super::runner;

    mod alpha {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn slash() {
            assert_eq!(
        runner().err(
            "a {b: color(srgb 0.1 0.2 0.3/0.4px)}\n"
        ),
        "Error: $alpha: Expected 0.4px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: color(srgb 0.1 0.2 0.3/0.4px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn slash_list() {
            assert_eq!(
        runner().err(
            "@use \'sass:list\';\
             \na {b: color(list.slash(srgb 0.1 0.2 0.3, 0.4px))}\n"
        ),
        "Error: $alpha: Expected 0.4px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: color(list.slash(srgb 0.1 0.2 0.3, 0.4px))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn blue() {
        assert_eq!(
            runner().err("a {b: color(srgb 0.1 0.2 0.3px)}\n"),
            "Error: $blue: Expected 0.3px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: color(srgb 0.1 0.2 0.3px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn green() {
        assert_eq!(
            runner().err("a {b: color(srgb 0.1 0.2px 0.3)}\n"),
            "Error: $green: Expected 0.2px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: color(srgb 0.1 0.2px 0.3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn red() {
        assert_eq!(
            runner().err("a {b: color(srgb 0.1px 0.2 0.3)}\n"),
            "Error: $red: Expected 0.1px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: color(srgb 0.1px 0.2 0.3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
#[ignore] // missing error
fn unknown_space() {
    assert_eq!(
        runner().err("a {b: color(foo 1 2 3)}\n"),
        "Error: $description: Unknown color space \"foo\".\
         \n  ,\
         \n1 | a {b: color(foo 1 2 3)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
