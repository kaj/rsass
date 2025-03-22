//! Tests auto-converted from "sass-spec/spec/core_functions/color/lch/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod list {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn bracketed() {
        assert_eq!(
            runner().err("a {b: lch([1% 2 3deg])}\n"),
            "Error: $channels: Expected an unbracketed list, was [1% 2 3deg]\
         \n  ,\
         \n1 | a {b: lch([1% 2 3deg])}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn comma() {
        assert_eq!(
        runner().err(
            "a {b: lch((1%, 2, 3deg))}\n"
        ),
        "Error: $channels: Expected a space- or slash-separated list, was (1%, 2, 3deg)\
         \n  ,\
         \n1 | a {b: lch((1%, 2, 3deg))}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn empty() {
        assert_eq!(
            runner().err("a {b: lch(())}\n"),
            "Error: $channels: Color component list may not be empty.\
         \n  ,\
         \n1 | a {b: lch(())}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn empty_space() {
        assert_eq!(
            runner().err(
                "@use \'sass:list\';\
             \n$empty-space: list.join((), (), $separator: space);\
             \na {b: lch(())}\n"
            ),
            "Error: $channels: Color component list may not be empty.\
         \n  ,\
         \n3 | a {b: lch(())}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
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
             \n$single-arg-slash-separated: list.append((), 1% 2 3deg, $separator: slash);\
             \na {b: lch($single-arg-slash-separated)}\n"
        ),
        "Error: $channels: Only 2 slash-separated elements allowed, but 1 was passed.\
         \n  ,\
         \n3 | a {b: lch($single-arg-slash-separated)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: lch(list.slash(1%, 2, 3deg))}\n"
        ),
        "Error: $channels: Only 2 slash-separated elements allowed, but 3 were passed.\
         \n  ,\
         \n2 | a {b: lch(list.slash(1%, 2, 3deg))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
            "a {b: lch(1% 2)}\n"
        ),
        "Error: $channels: The lch color space has 3 channels but (1% 2) has 2.\
         \n  ,\
         \n1 | a {b: lch(1% 2)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn too_many_channels() {
        assert_eq!(
        runner().err(
            "a {b: lch(1% 2 3deg 0.4)}\n"
        ),
        "Error: $channels: The lch color space has 3 channels but (1% 2 3deg 0.4) has 4.\
         \n  ,\
         \n1 | a {b: lch(1% 2 3deg 0.4)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
#[test]
#[ignore] // missing error
fn too_few_args() {
    assert_eq!(
        runner().err("a {b: lch()}\n"),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n1 | a {b: lch()}\
         \n  |       ^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function lch($channels) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn too_many_args() {
    assert_eq!(
        runner().err("a {b: lch(1%, 2, 3deg, 0.4)}\n"),
        "Error: Only 1 argument allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: lch(1%, 2, 3deg, 0.4)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function lch($channels) {\
         \n  |           ============== declaration\
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
             \na {b: lch(list.slash(1% 2 3deg, c))}\n"
                ),
                "Error: $channels: c is not a number.\
         \n  ,\
         \n2 | a {b: lch(list.slash(1% 2 3deg, c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn chroma() {
        assert_eq!(
        runner().err(
            "a {b: lch(1% c 3deg)}\n"
        ),
        "Error: $channels: Expected chroma channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: lch(1% c 3deg)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn hue() {
        assert_eq!(
            runner().err("a {b: lch(1% 2 c)}\n"),
            "Error: $channels: Expected hue channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: lch(1% 2 c)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lightness() {
        assert_eq!(
        runner().err(
            "a {b: lch(c 2 3deg)}\n"
        ),
        "Error: $channels: Expected lightness channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: lch(c 2 3deg)}\
         \n  |       ^^^^^^^^^^^^^\
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
            "a {b: lch(1% 2 3deg/0.4px)}\n"
        ),
        "Error: $alpha: Expected 0.4px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: lch(1% 2 3deg/0.4px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: lch(list.slash(1% 2 3deg, 0.4px))}\n"
        ),
        "Error: $alpha: Expected 0.4px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: lch(list.slash(1% 2 3deg, 0.4px))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn chroma() {
        assert_eq!(
            runner().err("a {b: lch(1% 2px 3deg)}\n"),
            "Error: $chroma: Expected 2px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: lch(1% 2px 3deg)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod hue {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn percent() {
            assert_eq!(
        runner().err(
            "a {b: lch(1% 2 3%)}\n"
        ),
        "Error: $hue: Expected 3% to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n1 | a {b: lch(1% 2 3%)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn unrelated() {
            assert_eq!(
        runner().err(
            "a {b: lch(1% 2 3px)}\n"
        ),
        "Error: $hue: Expected 3px to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n1 | a {b: lch(1% 2 3px)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn lightness() {
        assert_eq!(
            runner().err("a {b: lch(1px 2 3deg)}\n"),
            "Error: $lightness: Expected 1px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: lch(1px 2 3deg)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
