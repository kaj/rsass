//! Tests auto-converted from "sass-spec/spec/core_functions/list/separator.hrx"

mod empty {
    #[test]
    fn comma() {
        assert_eq!(
            crate::rsass(
                "a {b: list-separator(join((), (), comma))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: comma;\
        \n}\
        \n"
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \n\
            \na {b: list-separator($empty-map)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: space;\
        \n}\
        \n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            crate::rsass(
                "a {b: list-separator(())}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: space;\
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
                "a {b: list-separator()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $list.\
         \n  ,--> input.scss\
         \n1 | a {b: list-separator()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function separator($list) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: list-separator(c, d)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: list-separator(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function separator($list) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
mod multi {
    #[test]
    fn comma() {
        assert_eq!(
            crate::rsass(
                "a {b: list-separator((1, 2, 3))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: comma;\
        \n}\
        \n"
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            crate::rsass(
                "a {b: list-separator((c: d, e: f, g: h))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: comma;\
        \n}\
        \n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            crate::rsass(
                "a {b: list-separator(1 2 3)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: space;\
        \n}\
        \n"
        );
    }
}
mod single {
    #[test]
    fn comma() {
        assert_eq!(
            crate::rsass(
                "a {b: list-separator((1,))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: comma;\
        \n}\
        \n"
        );
    }
    #[test]
    fn non_list() {
        assert_eq!(
            crate::rsass(
                "a {b: list-separator(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: space;\
        \n}\
        \n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            crate::rsass(
                "a {b: list-separator([1])}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: space;\
        \n}\
        \n"
        );
    }
}
