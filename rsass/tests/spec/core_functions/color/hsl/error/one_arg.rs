//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/error/one_arg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("one_arg")
}

mod list {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn bracketed() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: hsl([0 100% 50%]);\
             \n}\n"
        ),
        "Error: $channels: Expected an unbracketed list, was [0 100% 50%]\
         \n  ,\
         \n2 |   b: hsl([0 100% 50%]);\
         \n  |      ^^^^^^^^^^^^^^^^^\
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
             \n  b: hsl((0, 100%, 50%));\
             \n}\n"
        ),
        "Error: $channels: Expected a space- or slash-separated list, was (0, 100%, 50%)\
         \n  ,\
         \n2 |   b: hsl((0, 100%, 50%));\
         \n  |      ^^^^^^^^^^^^^^^^^^^\
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
             \n  b: hsl(());\
             \n}\n"
            ),
            "Error: $channels: Color component list may not be empty.\
         \n  ,\
         \n2 |   b: hsl(());\
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
             \n  b: hsl(0 100% 50% 0.4);\
             \n}\n"
        ),
        "Error: $channels: The hsl color space has 3 channels but (0 100% 50% 0.4) has 4.\
         \n  ,\
         \n2 |   b: hsl(0 100% 50% 0.4);\
         \n  |      ^^^^^^^^^^^^^^^^^^^\
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
             \n  b: hsl(0);\
             \n}\n"
        ),
        "Error: $channels: The hsl color space has 3 channels but 0 has 1.\
         \n  ,\
         \n2 |   b: hsl(0);\
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
             \n  b: hsl(0 100%);\
             \n}\n"
        ),
        "Error: $channels: The hsl color space has 3 channels but (0 100%) has 2.\
         \n  ,\
         \n2 |   b: hsl(0 100%);\
         \n  |      ^^^^^^^^^^^\
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
             \n  b: hsl(0 100% \"var(--foo) / 0.4\");\
             \n}\n"
        ),
        "Error: $channels: Expected lightness channel to be a number, was \"var(--foo) / 0.4\".\
         \n  ,\
         \n2 |   b: hsl(0 100% \"var(--foo) / 0.4\");\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
mod slash_list {
    #[allow(unused)]
    use super::runner;

    mod channels {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn bracketed() {
            assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: hsl(list.slash([0 100% 50%], 1))}\n"
        ),
        "Error: $channels: Expected an unbracketed list, was [0 100% 50%]\
         \n  ,\
         \n2 | a {b: hsl(list.slash([0 100% 50%], 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: hsl(list.slash((0, 100%, 50%), 1))}\n"
        ),
        "Error: $channels: Expected a space-separated list, was (0, 100%, 50%)\
         \n  ,\
         \n2 | a {b: hsl(list.slash((0, 100%, 50%), 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: hsl(list.slash((), 1))}\n"
                ),
                "Error: $channels: Color component list may not be empty.\
         \n  ,\
         \n2 | a {b: hsl(list.slash((), 1))}\
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
             \na {b: hsl(list.slash(0 100% 50% 0.4, 1))}\n"
        ),
        "Error: $channels: The hsl color space has 3 channels but (0 100% 50% 0.4 / 1) has 4.\
         \n  ,\
         \n2 | a {b: hsl(list.slash(0 100% 50% 0.4, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: hsl(list.slash(0, 1))}\n"
        ),
        "Error: $channels: The hsl color space has 3 channels but (0 / 1) has 1.\
         \n  ,\
         \n2 | a {b: hsl(list.slash(0, 1))}\
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
             \na {b: hsl(list.slash(0 100%, 1))}\n"
        ),
        "Error: $channels: The hsl color space has 3 channels but (0 100% / 1) has 2.\
         \n  ,\
         \n2 | a {b: hsl(list.slash(0 100%, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: hsl(list.append((), 0 100% 100%, $separator: slash))}\n"
        ),
        "Error: $channels: Only 2 slash-separated elements allowed, but 1 was passed.\
         \n  ,\
         \n2 | a {b: hsl(list.append((), 0 100% 100%, $separator: slash))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: hsl(list.slash(0 100% 100%, 0.4, 1))}\n"
        ),
        "Error: $channels: Only 2 slash-separated elements allowed, but 3 were passed.\
         \n  ,\
         \n2 | a {b: hsl(list.slash(0 100% 100%, 0.4, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
mod test_type {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn hue() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: hsl(\"foo\" 100% 50%);\
             \n}\n"
        ),
        "Error: $channels: Expected hue channel to be a number, was \"foo\".\
         \n  ,\
         \n2 |   b: hsl(\"foo\" 100% 50%);\
         \n  |      ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn lightness() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: hsl(0 100% \"foo\");\
             \n}\n"
        ),
        "Error: $channels: Expected lightness channel to be a number, was \"foo\".\
         \n  ,\
         \n2 |   b: hsl(0 100% \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn saturation() {
        assert_eq!(
        runner().err(
            "a {\
             \n  b: hsl(0 \"foo\" 50%);\
             \n}\n"
        ),
        "Error: $channels: Expected saturation channel to be a number, was \"foo\".\
         \n  ,\
         \n2 |   b: hsl(0 \"foo\" 50%);\
         \n  |      ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
    }
}
