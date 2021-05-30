//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/error/one_arg.hrx"

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
             \n  b: rgba([1 2 3]);\
             \n}\n"
            ),
            "Error: $channels must be an unbracketed list.\
         \n  ,\
         \n2 |   b: rgba([1 2 3]);\
         \n  |      ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn comma_separated() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba((1, 2, 3));\
             \n}\n"
            ),
            "Error: $channels must be a space-separated list.\
         \n  ,\
         \n2 |   b: rgba((1, 2, 3));\
         \n  |      ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn empty() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(());\
             \n}\n"
            ),
            "Error: Missing element $red.\
         \n  ,\
         \n2 |   b: rgba(());\
         \n  |      ^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn four_elements() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(1 2 3 0.4);\
             \n}\n"
            ),
            "Error: Only 3 elements allowed, but 4 were passed.\
         \n  ,\
         \n2 |   b: rgba(1 2 3 0.4);\
         \n  |      ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn one_element() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(1);\
             \n}\n"
            ),
            "Error: Missing element $green.\
         \n  ,\
         \n2 |   b: rgba(1);\
         \n  |      ^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn two_elements() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(1 2);\
             \n}\n"
            ),
            "Error: Missing element $blue.\
         \n  ,\
         \n2 |   b: rgba(1 2);\
         \n  |      ^^^^^^^^^\
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
             \n  b: rgba(1 2 \"var(--foo) / 0.4\");\
             \n}\n"
        ),
        "Error: $blue: \"var(--foo) / 0.4\" is not a number.\
         \n  ,\
         \n2 |   b: rgba(1 2 \"var(--foo) / 0.4\");\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
        #[ignore] // missing error
        fn bracketed() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgba(list.slash([1 2 3], 1))}\n"
                ),
                "Error: $channels must be an unbracketed list.\
         \n  ,\
         \n2 | a {b: rgba(list.slash([1 2 3], 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn comma_separated() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgba(list.slash((1, 2, 3), 1))}\n"
                ),
                "Error: $channels must be a space-separated list.\
         \n  ,\
         \n2 | a {b: rgba(list.slash((1, 2, 3), 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn empty() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgba(list.slash((), 1))}\n"
                ),
                "Error: Missing element $red.\
         \n  ,\
         \n2 | a {b: rgba(list.slash((), 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn four_elements() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgba(list.slash(1 2 3 0.4, 1))}\n"
                ),
                "Error: Only 3 elements allowed, but 4 were passed.\
         \n  ,\
         \n2 | a {b: rgba(list.slash(1 2 3 0.4, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn one_element() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgba(list.slash(1, 1))}\n"
                ),
                "Error: Missing element $green.\
         \n  ,\
         \n2 | a {b: rgba(list.slash(1, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn two_elements() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: rgba(list.slash(1 2, 1))}\n"
                ),
                "Error: Missing element $blue.\
         \n  ,\
         \n2 | a {b: rgba(list.slash(1 2, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn too_few_elements() {
        assert_eq!(
        runner().err(
            "a {b: rgba(append((), 1 2 3, $separator: slash))}\n"
        ),
        "Error: Only 2 slash-separated elements allowed, but 1 was passed.\
         \n  ,\
         \n1 | a {b: rgba(append((), 1 2 3, $separator: slash))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_elements() {
        assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: rgba(list.slash(1 2 3, 0.4, 1))}\n"
        ),
        "Error: Only 2 slash-separated elements allowed, but 3 were passed.\
         \n  ,\
         \n2 | a {b: rgba(list.slash(1 2 3, 0.4, 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \n  b: rgba(1 2 \"foo\");\
             \n}\n"
            ),
            "Error: $blue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgba(1 2 \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(1 \"foo\" 3);\
             \n}\n"
            ),
            "Error: $green: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgba(1 \"foo\" 3);\
         \n  |      ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(\"foo\" 2 3);\
             \n}\n"
            ),
            "Error: $red: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgba(\"foo\" 2 3);\
         \n  |      ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
