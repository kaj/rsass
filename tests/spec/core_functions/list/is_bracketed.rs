//! Tests auto-converted from "sass-spec/spec/core_functions/list/is_bracketed.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("is_bracketed")
}

mod bracketed {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("a {b: is-bracketed([])}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn multi() {
        assert_eq!(
            runner().ok("a {b: is-bracketed([1, 2, 3])}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("a {b: is-bracketed([1])}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: is-bracketed()}\n"),
            "Error: Missing argument $list.\
         \n  ,--> input.scss\
         \n1 | a {b: is-bracketed()}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function is-bracketed($list) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: is-bracketed(a b, c d)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: is-bracketed(a b, c d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function is-bracketed($list) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod unbracketed {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("a {b: is-bracketed(())}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            runner().ok("a {b: is-bracketed((c: d, e: f, g: h))}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn multi() {
        assert_eq!(
            runner().ok("a {b: is-bracketed(1 2 3)}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn non_list() {
        assert_eq!(
            runner().ok("a {b: is-bracketed(1)}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("a {b: is-bracketed((1,))}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
