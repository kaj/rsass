//! Tests auto-converted from "sass-spec/spec/core_functions/string/quote.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("quote")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.quote()}\n"
            ),
            "Error: Missing argument $string.\
         \n  ,--> input.scss\
         \n2 | a {b: string.quote()}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function quote($string) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.quote(c, d)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: string.quote(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function quote($string) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.quote((1, 2, 3))}\n"
            ),
            "Error: $string: (1, 2, 3) is not a string.\
         \n  ,\
         \n2 | a {b: string.quote((1, 2, 3))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn escape() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.quote(\\0)}\n"),
        "a {\
         \n  b: \"\\\\0 \";\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.quote($string: c)}\n"),
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
        runner().ok(
            "@use \"sass:string\";\
             \n// See sass/libsass#2873\
             \na {b: string.quote(string.unquote(\'\"\') + string.unquote(\"\'\"))}\n"
        ),
        "a {\
         \n  b: \"\\\"\'\";\
         \n}\n"
    );
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \n// See sass/libsass#2873\
             \na {b: string.quote(string.unquote(\'\"\'))}\n"),
            "a {\
         \n  b: \'\"\';\
         \n}\n"
        );
    }
}
#[test]
fn quoted_double() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.quote(\"c\")}\n"),
        "a {\
         \n  b: \"c\";\
         \n}\n"
    );
}
#[test]
fn quoted_single() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.quote(\'c\')}\n"),
        "a {\
         \n  b: \"c\";\
         \n}\n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.quote(c)}\n"),
        "a {\
         \n  b: \"c\";\
         \n}\n"
    );
}
