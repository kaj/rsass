//! Tests auto-converted from "sass-spec/spec/core_functions/color/oklab/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod list {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn bracketed() {
        assert_eq!(
            runner().err("a {b: oklab([1% 2 3])}\n"),
            "Error: $channels: Expected an unbracketed list, was [1% 2 3]\
         \n  ,\
         \n1 | a {b: oklab([1% 2 3])}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn comma() {
        assert_eq!(
        runner().err(
            "a {b: oklab((1%, 2, 3))}\n"
        ),
        "Error: $channels: Expected a space- or slash-separated list, was (1%, 2, 3)\
         \n  ,\
         \n1 | a {b: oklab((1%, 2, 3))}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn empty() {
        assert_eq!(
            runner().err("a {b: oklab(())}\n"),
            "Error: $channels: Color component list may not be empty.\
         \n  ,\
         \n1 | a {b: oklab(())}\
         \n  |       ^^^^^^^^^\
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
             \na {b: oklab(())}\n"
            ),
            "Error: $channels: Color component list may not be empty.\
         \n  ,\
         \n3 | a {b: oklab(())}\
         \n  |       ^^^^^^^^^\
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
             \n$single-arg-slash-separated: list.append((), 1% 2 3, $separator: slash);\
             \na {b: oklab($single-arg-slash-separated)}\n"
        ),
        "Error: $channels: Only 2 slash-separated elements allowed, but 1 was passed.\
         \n  ,\
         \n3 | a {b: oklab($single-arg-slash-separated)}\
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
             \na {b: oklab(list.slash(1%, 2, 3))}\n"
        ),
        "Error: $channels: Only 2 slash-separated elements allowed, but 3 were passed.\
         \n  ,\
         \n2 | a {b: oklab(list.slash(1%, 2, 3))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
            "a {b: oklab(1% 2)}\n"
        ),
        "Error: $channels: The oklab color space has 3 channels but (1% 2) has 2.\
         \n  ,\
         \n1 | a {b: oklab(1% 2)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn too_many_channels() {
        assert_eq!(
        runner().err(
            "a {b: oklab(1% 2 3 0.4)}\n"
        ),
        "Error: $channels: The oklab color space has 3 channels but (1% 2 3 0.4) has 4.\
         \n  ,\
         \n1 | a {b: oklab(1% 2 3 0.4)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
#[test]
#[ignore] // missing error
fn too_few_args() {
    assert_eq!(
        runner().err("a {b: oklab()}\n"),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n1 | a {b: oklab()}\
         \n  |       ^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function oklab($channels) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn too_many_args() {
    assert_eq!(
        runner().err("a {b: oklab(1%, 2, 3, 0.4)}\n"),
        "Error: Only 1 argument allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: oklab(1%, 2, 3, 0.4)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function oklab($channels) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
mod test_type {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn a() {
        assert_eq!(
            runner().err("a {b: oklab(1% c 3)}\n"),
            "Error: $channels: Expected a channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: oklab(1% c 3)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod alpha {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn slash_list() {
            assert_eq!(
                runner().err(
                    "@use \'sass:list\';\
             \na {b: oklab(list.slash(1% 2 3, c))}\n"
                ),
                "Error: $channels: c is not a number.\
         \n  ,\
         \n2 | a {b: oklab(list.slash(1% 2 3, c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn b() {
        assert_eq!(
            runner().err("a {b: oklab(1% 2 c)}\n"),
            "Error: $channels: Expected b channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: oklab(1% 2 c)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lightness() {
        assert_eq!(
        runner().err(
            "a {b: oklab(c 2 3)}\n"
        ),
        "Error: $channels: Expected lightness channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: oklab(c 2 3)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
mod unit {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn a() {
        assert_eq!(
            runner().err("a {b: oklab(1% 2px 3)}\n"),
            "Error: $a: Expected 2px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: oklab(1% 2px 3)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod alpha {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn slash() {
            assert_eq!(
        runner().err(
            "a {b: oklab(1% 2 3/0.4px)}\n"
        ),
        "Error: $alpha: Expected 0.4px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: oklab(1% 2 3/0.4px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
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
             \na {b: oklab(list.slash(1% 2 3, 0.4px))}\n"
        ),
        "Error: $alpha: Expected 0.4px to have unit \"%\" or no units.\
         \n  ,\
         \n2 | a {b: oklab(list.slash(1% 2 3, 0.4px))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn b() {
        assert_eq!(
            runner().err("a {b: oklab(1% 2 3px)}\n"),
            "Error: $b: Expected 3px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: oklab(1% 2 3px)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lightness() {
        assert_eq!(
            runner().err("a {b: oklab(1px 2 3)}\n"),
            "Error: $lightness: Expected 1px to have unit \"%\" or no units.\
         \n  ,\
         \n1 | a {b: oklab(1px 2 3)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
