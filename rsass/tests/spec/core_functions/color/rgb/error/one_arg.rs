//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/error/one_arg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("one_arg")
}

mod list {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn bracketed() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb([1 2 3]);\
             \n}\n"
            ),
            "Error: $channels must be an unbracketed list.\
         \n  ,\
         \n2 |   b: rgb([1 2 3]);\
         \n  |      ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn comma_separated() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb((1, 2, 3));\
             \n}\n"
            ),
            "Error: $channels must be a space-separated list.\
         \n  ,\
         \n2 |   b: rgb((1, 2, 3));\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn empty() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(());\
             \n}\n"
            ),
            "Error: Missing element $red.\
         \n  ,\
         \n2 |   b: rgb(());\
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
             \n  b: rgb(1 2 3 0.4);\
             \n}\n"
            ),
            "Error: Only 3 elements allowed, but 4 were passed.\
         \n  ,\
         \n2 |   b: rgb(1 2 3 0.4);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn one_element() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(1);\
             \n}\n"
            ),
            "Error: Missing element $green.\
         \n  ,\
         \n2 |   b: rgb(1);\
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
             \n  b: rgb(1 2);\
             \n}\n"
            ),
            "Error: Missing element $blue.\
         \n  ,\
         \n2 |   b: rgb(1 2);\
         \n  |      ^^^^^^^^\
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
             \n  b: rgb(1 2 \"var(--foo) / 0.4\");\
             \n}\n"
        ),
        "Error: $blue: \"var(--foo) / 0.4\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(1 2 \"var(--foo) / 0.4\");\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: rgb(list.slash([1 2 3], 1))}\n"
                ),
                "Error: $channels must be an unbracketed list.\
         \n  ,\
         \n2 | a {b: rgb(list.slash([1 2 3], 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn comma_separated() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgb(list.slash((1, 2, 3), 1))}\n"
                ),
                "Error: $channels must be a space-separated list.\
         \n  ,\
         \n2 | a {b: rgb(list.slash((1, 2, 3), 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn empty() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgb(list.slash((), 1))}\n"
                ),
                "Error: Missing element $red.\
         \n  ,\
         \n2 | a {b: rgb(list.slash((), 1))}\
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
             \na {b: rgb(list.slash(1 2 3 0.4, 1))}\n"
                ),
                "Error: Only 3 elements allowed, but 4 were passed.\
         \n  ,\
         \n2 | a {b: rgb(list.slash(1 2 3 0.4, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn one_element() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgb(list.slash(1, 1))}\n"
                ),
                "Error: Missing element $green.\
         \n  ,\
         \n2 | a {b: rgb(list.slash(1, 1))}\
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
             \na {b: rgb(list.slash(1 2, 1))}\n"
                ),
                "Error: Missing element $blue.\
         \n  ,\
         \n2 | a {b: rgb(list.slash(1 2, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_elements() {
        assert_eq!(
        runner().err(
            "a {b: rgb(append((), 1 2 3, $separator: slash))}\n"
        ),
        "Error: Only 2 slash-separated elements allowed, but 1 was passed.\
         \n  ,\
         \n1 | a {b: rgb(append((), 1 2 3, $separator: slash))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_many_elements() {
        assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: rgb(list.slash(1 2 3, 0.4, 1))}\n"
        ),
        "Error: Only 2 slash-separated elements allowed, but 3 were passed.\
         \n  ,\
         \n2 | a {b: rgb(list.slash(1 2 3, 0.4, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
mod test_type {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn blue() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(1 2 \"foo\");\
             \n}\n"
            ),
            "Error: $blue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(1 2 \"foo\");\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(1 \"foo\" 3);\
             \n}\n"
            ),
            "Error: $green: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(1 \"foo\" 3);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(\"foo\" 2 3);\
             \n}\n"
            ),
            "Error: $red: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(\"foo\" 2 3);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
