//! Tests auto-converted from "sass-spec/spec/core_functions/list/is_bracketed.hrx"

mod bracketed {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed([])}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
    #[test]
    fn multi() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed([1, 2, 3])}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed([1])}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $list.\
         \n  ,--> input.scss\
         \n1 | a {b: is-bracketed()}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function is-bracketed($list) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed(a b, c d)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: is-bracketed(a b, c d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function is-bracketed($list) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
mod unbracketed {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed(())}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed((c: d, e: f, g: h))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
    #[test]
    fn multi() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed(1 2 3)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
    #[test]
    fn non_list() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: is-bracketed((1,))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
}
