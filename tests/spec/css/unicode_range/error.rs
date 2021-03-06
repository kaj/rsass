//! Tests auto-converted from "sass-spec/spec/css/unicode_range/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn ident_minus_space_ident() {
    assert_eq!(
        runner().err(
            ".ident-minus-space-ident {\
             \n  a: U+abc- def;\
             \n}\n"
        ),
        "Error: Expected hex digit.\
         \n  ,\
         \n2 |   a: U+abc- def;\
         \n  |            ^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn minus_ident_minus() {
    assert_eq!(
        runner().err(
            ".minus-ident-minus {\
             \n  a: u+123-abc-def;\
             \n}\n"
        ),
        "Error: Expected end of identifier.\
         \n  ,\
         \n2 |   a: u+123-abc-def;\
         \n  |               ^\
         \n  \'\
         \n  input.scss 2:15  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn minus_number_minus_ident() {
    assert_eq!(
        runner().err(
            ".minus-number-minus-ident {\
             \n  a: U+123-456-ABC;\
             \n}\n"
        ),
        "Error: Expected end of identifier.\
         \n  ,\
         \n2 |   a: U+123-456-ABC;\
         \n  |               ^\
         \n  \'\
         \n  input.scss 2:15  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn no_digits() {
    assert_eq!(
        runner().err(
            ".no-digits {\
             \n  a: U+;\
             \n}\n"
        ),
        "Error: Expected hex digit or \"?\".\
         \n  ,\
         \n2 |   a: U+;\
         \n  |        ^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn nothing_after_minus() {
    assert_eq!(
        runner().err(
            ".nothing-after-minus {\
             \n  a: U+abc-;\
             \n}\n"
        ),
        "Error: Expected hex digit.\
         \n  ,\
         \n2 |   a: U+abc-;\
         \n  |            ^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn question_mark_after_minus() {
    assert_eq!(
        runner().err(
            ".question-mark-after-minus {\
             \n  a: u+abc-de?;\
             \n}\n"
        ),
        "Error: expected \";\".\
         \n  ,\
         \n2 |   a: u+abc-de?;\
         \n  |              ^\
         \n  \'\
         \n  input.scss 2:14  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn too_many_decimal_digits() {
    assert_eq!(
        runner().err(
            ".too-many-decimal-digits {\
             \n  a: U+1234567;\
             \n}\n"
        ),
        "Error: Expected end of identifier.\
         \n  ,\
         \n2 |   a: U+1234567;\
         \n  |              ^\
         \n  \'\
         \n  input.scss 2:14  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn too_many_decimal_digits_after_minus() {
    assert_eq!(
        runner().err(
            ".too-many-decimal-digits-after-minus {\
             \n  a: U+abc-1234567;\
             \n}\n"
        ),
        "Error: Expected end of identifier.\
         \n  ,\
         \n2 |   a: U+abc-1234567;\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 2:18  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn too_many_digits_after_minus() {
    assert_eq!(
        runner().err(
            ".too-many-hex-digits-after-minus {\
             \n  a: U+abc-abcdefa;\
             \n}\n"
        ),
        "Error: Expected end of identifier.\
         \n  ,\
         \n2 |   a: U+abc-abcdefa;\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 2:18  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn too_many_hex_digits() {
    assert_eq!(
        runner().err(
            ".too-many-hex-digits {\
             \n  a: U+ABCDEFA;\
             \n}\n"
        ),
        "Error: Expected end of identifier.\
         \n  ,\
         \n2 |   a: U+ABCDEFA;\
         \n  |              ^\
         \n  \'\
         \n  input.scss 2:14  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn too_many_question_marks() {
    assert_eq!(
        runner().err(
            ".too-many-question-marks {\
             \n  a: U+???????;\
             \n}\n"
        ),
        "Error: expected \";\".\
         \n  ,\
         \n2 |   a: U+???????;\
         \n  |              ^\
         \n  \'\
         \n  input.scss 2:14  root stylesheet",
    );
}
