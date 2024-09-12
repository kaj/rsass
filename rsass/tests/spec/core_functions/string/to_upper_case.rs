//! Tests auto-converted from "sass-spec/spec/core_functions/string/to_upper_case.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("to_upper_case")
}

#[test]
fn alphabet() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \na {b: string.to-upper-case(\"abcdefghijklmnopqrstuvqxyz\")}\n"
        ),
        "a {\
         \n  b: \"ABCDEFGHIJKLMNOPQRSTUVQXYZ\";\
         \n}\n"
    );
}
#[test]
fn empty() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.to-upper-case(\"\")}\n"),
        "a {\
         \n  b: \"\";\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.to-upper-case()}\n"
            ),
            "Error: Missing argument $string.\
         \n  ,--> input.scss\
         \n2 | a {b: string.to-upper-case()}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function to-upper-case($string) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.to-upper-case(\"\", \"\")}\n\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: string.to-upper-case(\"\", \"\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function to-upper-case($string) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.to-upper-case(1)}\n"
            ),
            "Error: $string: 1 is not a string.\
         \n  ,\
         \n2 | a {b: string.to-upper-case(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.to-upper-case($string: abcDEF)}\n"),
        "a {\
         \n  b: ABCDEF;\
         \n}\n"
    );
}
#[test]
fn non_ascii() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \n// Only ASCII characters have their case changed.\
             \na {b: string.to-upper-case(\"äçðøþ\")}\n"),
        "@charset \"UTF-8\";\
         \na {\
         \n  b: \"äçðøþ\";\
         \n}\n"
    );
}
#[test]
fn number() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.to-upper-case(\"1234567890\")}\n"),
        "a {\
         \n  b: \"1234567890\";\
         \n}\n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.to-upper-case(aBcDeF)}\n"),
        "a {\
         \n  b: ABCDEF;\
         \n}\n"
    );
}
