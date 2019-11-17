//! Tests auto-converted from "sass-spec/spec/core_functions/string/to_lower_case.hrx"

#[test]
fn alphabet() {
    assert_eq!(
        crate::rsass(
            "a {b: to-lower-case(\"ABCDEFGHIJKLMNOPQRSTUVQXYZ\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"abcdefghijklmnopqrstuvqxyz\";\
        \n}\
        \n"
    );
}
#[test]
fn empty() {
    assert_eq!(
        crate::rsass(
            "a {b: to-lower-case(\"\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"\";\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: to-lower-case()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $string.\
         \n  ,--> input.scss\
         \n1 | a {b: to-lower-case()}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function to-lower-case($string) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: to-lower-case(\"\", \"\")}\
             \n\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: to-lower-case(\"\", \"\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function to-lower-case($string) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: to-lower-case(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $string: 1 is not a string.\
         \n  ,\
         \n1 | a {b: to-lower-case(1)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: to-lower-case($string: abcDEF)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: abcdef;\
        \n}\
        \n"
    );
}
#[test]
fn non_ascii() {
    assert_eq!(
        crate::rsass(
            "// Only ASCII characters have their case changed.\
            \na {b: to-lower-case(\"ÄÇÐØÞ\")}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \na {\
        \n  b: \"ÄÇÐØÞ\";\
        \n}\
        \n"
    );
}
#[test]
fn number() {
    assert_eq!(
        crate::rsass(
            "a {b: to-lower-case(\"1234567890\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"1234567890\";\
        \n}\
        \n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        crate::rsass(
            "a {b: to-lower-case(aBcDeF)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: abcdef;\
        \n}\
        \n"
    );
}
