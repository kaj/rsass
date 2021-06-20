//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/error/one_arg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod list {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn bracketed() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl([0 100% 50%]);\
             \n}\n"
            ),
            "Error: $channels must be an unbracketed list.\
         \n  ,\
         \n2 |   b: hsl([0 100% 50%]);\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn comma_separated() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl((0, 100%, 50%));\
             \n}\n"
            ),
            "Error: $channels must be a space-separated list.\
         \n  ,\
         \n2 |   b: hsl((0, 100%, 50%));\
         \n  |      ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn empty() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(());\
             \n}\n"
            ),
            "Error: Missing element $hue.\
         \n  ,\
         \n2 |   b: hsl(());\
         \n  |      ^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn four_elements() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(0 100% 50% 0.4);\
             \n}\n"
            ),
            "Error: Only 3 elements allowed, but 4 were passed.\
         \n  ,\
         \n2 |   b: hsl(0 100% 50% 0.4);\
         \n  |      ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn one_element() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(0);\
             \n}\n"
            ),
            "Error: Missing element $saturation.\
         \n  ,\
         \n2 |   b: hsl(0);\
         \n  |      ^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn two_elements() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(0 100%);\
             \n}\n"
            ),
            "Error: Missing element $lightness.\
         \n  ,\
         \n2 |   b: hsl(0 100%);\
         \n  |      ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
#[test]
fn quoted_var_slash() {
    assert_eq!(
        runner().err(
            "a {\
             \n  b: hsl(0 100% \"var(--foo) / 0.4\");\
             \n}\n"
        ),
        "Error: $lightness: \"var(--foo) / 0.4\" is not a number.\
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
        fn bracketed() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: hsl(list.slash([0 100% 50%], 1))}\n"
                ),
                "Error: $channels must be an unbracketed list.\
         \n  ,\
         \n2 | a {b: hsl(list.slash([0 100% 50%], 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn comma_separated() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: hsl(list.slash((0, 100%, 50%), 1))}\n"
                ),
                "Error: $channels must be a space-separated list.\
         \n  ,\
         \n2 | a {b: hsl(list.slash((0, 100%, 50%), 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn empty() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: hsl(list.slash((), 1))}\n"
                ),
                "Error: Missing element $hue.\
         \n  ,\
         \n2 | a {b: hsl(list.slash((), 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn four_elements() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: hsl(list.slash(0 100% 50% 0.4, 1))}\n"
                ),
                "Error: Only 3 elements allowed, but 4 were passed.\
         \n  ,\
         \n2 | a {b: hsl(list.slash(0 100% 50% 0.4, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn one_element() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: hsl(list.slash(0, 1))}\n"
                ),
                "Error: Missing element $saturation.\
         \n  ,\
         \n2 | a {b: hsl(list.slash(0, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn two_elements() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: hsl(list.slash(0 100%, 1))}\n"
                ),
                "Error: Missing element $lightness.\
         \n  ,\
         \n2 | a {b: hsl(list.slash(0 100%, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_elements() {
        assert_eq!(
        runner().err(
            "a {b: hsl(append((), 0 100% 100%, $separator: slash))}\n"
        ),
        "Error: Only 2 slash-separated elements allowed, but 1 was passed.\
         \n  ,\
         \n1 | a {b: hsl(append((), 0 100% 100%, $separator: slash))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_many_elements() {
        assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: hsl(list.slash(0 100% 100%, 0.4, 1))}\n"
        ),
        "Error: Only 2 slash-separated elements allowed, but 3 were passed.\
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
    fn hue() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(\"foo\" 100% 50%);\
             \n}\n"
            ),
            "Error: $hue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsl(\"foo\" 100% 50%);\
         \n  |      ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn lightness() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(0 100% \"foo\");\
             \n}\n"
            ),
            "Error: $lightness: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsl(0 100% \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn saturation() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(0 \"foo\" 50%);\
             \n}\n"
            ),
            "Error: $saturation: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsl(0 \"foo\" 50%);\
         \n  |      ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
