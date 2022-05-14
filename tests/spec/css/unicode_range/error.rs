//! Tests auto-converted from "sass-spec/spec/css/unicode_range/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

#[test]
#[ignore] // missing error
fn ident_minus_space_ident() {
    assert_eq!(
        runner().err("a {b: U+abc- def}\n"),
        "Error: Expected hex digit.\
         \n  ,\
         \n1 | a {b: U+abc- def}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn minus_ident_minus() {
    assert_eq!(
        runner().err("a {b: u+123-abc-def}\n"),
        "Error: Expected end of identifier.\
         \n  ,\
         \n1 | a {b: u+123-abc-def}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn minus_number_minus_ident() {
    assert_eq!(
        runner().err("a {b: U+123-456-ABC}\n"),
        "Error: Expected end of identifier.\
         \n  ,\
         \n1 | a {b: U+123-456-ABC}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn no_digits() {
    assert_eq!(
        runner().err("a {b: U+}\n"),
        "Error: Expected hex digit or \"?\".\
         \n  ,\
         \n1 | a {b: U+}\
         \n  |         ^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn nothing_after_minus() {
    assert_eq!(
        runner().err("a {b: U+abc-}\n"),
        "Error: Expected hex digit.\
         \n  ,\
         \n1 | a {b: U+abc-}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn question_mark_after_minus() {
    assert_eq!(
        runner().err("a {b: u+abc-de?}\n"),
        "Error: expected \";\".\
         \n  ,\
         \n1 | a {b: u+abc-de?}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
    );
}
mod too_many {
    #[allow(unused)]
    use super::runner;

    mod after_minus {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn decimal_digits() {
            assert_eq!(
                runner().err("a {b: U+abc-1234567}\n"),
                "Error: Expected at most 6 digits.\
         \n  ,\
         \n1 | a {b: U+abc-1234567}\
         \n  |             ^^^^^^^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn hex_digits() {
            assert_eq!(
                runner().err("a {b: U+abc-abcdefa}\n"),
                "Error: Expected at most 6 digits.\
         \n  ,\
         \n1 | a {b: U+abc-abcdefa}\
         \n  |             ^^^^^^^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn decimal_digits() {
        assert_eq!(
            runner().err("a {b: U+1234567}\n"),
            "Error: Expected at most 6 digits.\
         \n  ,\
         \n1 | a {b: U+1234567}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn hex_digits() {
        assert_eq!(
            runner().err("a {b: U+ABCDEFA}\n"),
            "Error: Expected at most 6 digits.\
         \n  ,\
         \n1 | a {b: U+ABCDEFA}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod question_marks {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn after_decimal() {
            assert_eq!(
                runner().err("a {b: U+12345??}\n"),
                "Error: Expected at most 6 digits.\
         \n  ,\
         \n1 | a {b: U+12345??}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn after_question_mark() {
            assert_eq!(
                runner().err("a {b: U+???????}\n"),
                "Error: Expected at most 6 digits.\
         \n  ,\
         \n1 | a {b: U+???????}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
