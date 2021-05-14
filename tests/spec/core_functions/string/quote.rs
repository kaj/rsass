//! Tests auto-converted from "sass-spec/spec/core_functions/string/quote.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: quote()}\n"),
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
            runner().err("a {b: quote(c, d)}\n"),
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
            runner().err("a {b: quote((1, 2, 3))}\n"),
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
        runner().ok("a {b: quote(\\0)}\n"),
        "a {\
         \n  b: \"\\\\0 \";\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: quote($string: c)}\n"),
        "a {\
         \n  b: \"c\";\
         \n}\n"
    );
}
mod quote_unquoted_quote {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn double() {
        assert_eq!(
            runner().ok("// See sass/libsass#2873\
             \na {b: quote(unquote(\'\"\') + unquote(\"\'\"))}\n"),
            "a {\
         \n  b: \"\\\"\'\";\
         \n}\n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("// See sass/libsass#2873\
             \na {b: quote(unquote(\'\"\'))}\n"),
            "a {\
         \n  b: \'\"\';\
         \n}\n"
        );
    }
}
#[test]
fn quoted_double() {
    assert_eq!(
        runner().ok("a {b: quote(\"c\")}\n"),
        "a {\
         \n  b: \"c\";\
         \n}\n"
    );
}
#[test]
fn quoted_single() {
    assert_eq!(
        runner().ok("a {b: quote(\'c\')}\n"),
        "a {\
         \n  b: \"c\";\
         \n}\n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        runner().ok("a {b: quote(c)}\n"),
        "a {\
         \n  b: \"c\";\
         \n}\n"
    );
}
