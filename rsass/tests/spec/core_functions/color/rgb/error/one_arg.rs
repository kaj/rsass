//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/error/one_arg.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("one_arg")
}

mod list {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn bracketed() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb([1 2 3]);\
             \n}\n"
            ),
            "Error: $channels: Expected an unbracketed list, was [1 2 3]\
         \n  ,\
         \n2 |   b: rgb([1 2 3]);\
         \n  |      ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn comma_separated() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: rgb((1, 2, 3));\
             \n}\n"
        ),
        "Error: $channels: Expected a space- or slash-separated list, was (1, 2, 3)\
         \n  ,\
         \n2 |   b: rgb((1, 2, 3));\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn empty() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(());\
             \n}\n"
            ),
            "Error: $channels: Color component list may not be empty.\
         \n  ,\
         \n2 |   b: rgb(());\
         \n  |      ^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn four_elements() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: rgb(1 2 3 0.4);\
             \n}\n"
        ),
        "Error: $channels: The rgb color space has 3 channels but (1 2 3 0.4) has 4.\
         \n  ,\
         \n2 |   b: rgb(1 2 3 0.4);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn one_element() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: rgb(1);\
             \n}\n"
        ),
        "Error: $channels: The rgb color space has 3 channels but 1 has 1.\
         \n  ,\
         \n2 |   b: rgb(1);\
         \n  |      ^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn two_elements() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: rgb(1 2);\
             \n}\n"
        ),
        "Error: $channels: The rgb color space has 3 channels but (1 2) has 2.\
         \n  ,\
         \n2 |   b: rgb(1 2);\
         \n  |      ^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong error
fn quoted_var_slash() {
    assert_eq!(
        runner().err(
            "a {\
             \n  b: rgb(1 2 \"var(--foo) / 0.4\");\
             \n}\n"
        ),
        "Error: $channels: Expected blue channel to be a number, was \"var(--foo) / 0.4\".\
         \n  ,\
         \n2 |   b: rgb(1 2 \"var(--foo) / 0.4\");\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
mod slash_list {
    use super::runner;

    mod channels {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn bracketed() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgb(list.slash([1 2 3], 1))}\n"
                ),
                "Error: $channels: Expected an unbracketed list, was [1 2 3]\
         \n  ,\
         \n2 | a {b: rgb(list.slash([1 2 3], 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn comma_separated() {
            assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: rgb(list.slash((1, 2, 3), 1))}\n"
        ),
        "Error: $channels: Expected a space-separated list, was (1, 2, 3)\
         \n  ,\
         \n2 | a {b: rgb(list.slash((1, 2, 3), 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn empty() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgb(list.slash((), 1))}\n"
                ),
                "Error: $channels: Color component list may not be empty.\
         \n  ,\
         \n2 | a {b: rgb(list.slash((), 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn four_elements() {
            assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: rgb(list.slash(1 2 3 0.4, 1))}\n"
        ),
        "Error: $channels: The rgb color space has 3 channels but (1 2 3 0.4 / 1) has 4.\
         \n  ,\
         \n2 | a {b: rgb(list.slash(1 2 3 0.4, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn one_element() {
            assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: rgb(list.slash(1, 1))}\n"
        ),
        "Error: $channels: The rgb color space has 3 channels but (1 / 1) has 1.\
         \n  ,\
         \n2 | a {b: rgb(list.slash(1, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn two_elements() {
            assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: rgb(list.slash(1 2, 1))}\n"
        ),
        "Error: $channels: The rgb color space has 3 channels but (1 2 / 1) has 2.\
         \n  ,\
         \n2 | a {b: rgb(list.slash(1 2, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_elements() {
        assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: rgb(list.append((), 1 2 3, $separator: slash))}\n"
        ),
        "Error: $channels: Only 2 slash-separated elements allowed, but 1 was passed.\
         \n  ,\
         \n2 | a {b: rgb(list.append((), 1 2 3, $separator: slash))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_elements() {
        assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: rgb(list.slash(1 2 3, 0.4, 1))}\n"
        ),
        "Error: $channels: Only 2 slash-separated elements allowed, but 3 were passed.\
         \n  ,\
         \n2 | a {b: rgb(list.slash(1 2 3, 0.4, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
mod test_type {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn blue() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: rgb(1 2 \"foo\");\
             \n}\n"
        ),
        "Error: $channels: Expected blue channel to be a number, was \"foo\".\
         \n  ,\
         \n2 |   b: rgb(1 2 \"foo\");\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn green() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: rgb(1 \"foo\" 3);\
             \n}\n"
        ),
        "Error: $channels: Expected green channel to be a number, was \"foo\".\
         \n  ,\
         \n2 |   b: rgb(1 \"foo\" 3);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn red() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: rgb(\"foo\" 2 3);\
             \n}\n"
        ),
        "Error: $channels: Expected red channel to be a number, was \"foo\".\
         \n  ,\
         \n2 |   b: rgb(\"foo\" 2 3);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
    }
}
