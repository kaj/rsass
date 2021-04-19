//! Tests auto-converted from "sass-spec/spec/core_functions/string/quote.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: quote()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $string.\
         \n  ,--> input.scss\
         \n1 | a {b: quote()}\
         \n  |       ^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function quote($string) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: quote(c, d)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: quote(c, d)}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function quote($string) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: quote((1, 2, 3))}\
             \n"
            )
            .unwrap_err(),
            "Error: $string: 1, 2, 3 is not a string.\
         \n  ,\
         \n1 | a {b: quote((1, 2, 3))}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn escape() {
    assert_eq!(
        crate::rsass(
            "a {b: quote(\\0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"\\\\0 \";\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: quote($string: c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"c\";\
        \n}\
        \n"
    );
}
mod quote_unquoted_quote {
    #[test]
    fn double() {
        assert_eq!(
            crate::rsass(
                "// See sass/libsass#2873\
            \na {b: quote(unquote(\'\"\') + unquote(\"\'\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"\\\"\'\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "// See sass/libsass#2873\
            \na {b: quote(unquote(\'\"\'))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \'\"\';\
        \n}\
        \n"
        );
    }
}
#[test]
fn quoted_double() {
    assert_eq!(
        crate::rsass(
            "a {b: quote(\"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"c\";\
        \n}\
        \n"
    );
}
#[test]
fn quoted_single() {
    assert_eq!(
        crate::rsass(
            "a {b: quote(\'c\')}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"c\";\
        \n}\
        \n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        crate::rsass(
            "a {b: quote(c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"c\";\
        \n}\
        \n"
    );
}
