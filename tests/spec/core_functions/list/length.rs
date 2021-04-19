//! Tests auto-converted from "sass-spec/spec/core_functions/list/length.hrx"

#[test]
fn t0() {
    assert_eq!(
        crate::rsass(
            "a {b: length(())}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
#[test]
fn t1() {
    assert_eq!(
        crate::rsass(
            "a {b: length(join((), 1))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn t2() {
    assert_eq!(
        crate::rsass(
            "a {b: length(c d)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2;\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: length()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $list.\
         \n  ,--> input.scss\
         \n1 | a {b: length()}\
         \n  |       ^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function length($list) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: length(1, 2)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: length(1, 2)}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function length($list) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn many() {
    assert_eq!(
        crate::rsass(
            "a {b: length((1, 2, 3, 4, 5))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 5;\
        \n}\
        \n"
    );
}
mod map {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \na {b: length($empty-map)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            crate::rsass(
                "a {b: length((1: 2, 3: 4))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: length($list: 1 2 3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3;\
        \n}\
        \n"
    );
}
#[test]
fn non_list() {
    assert_eq!(
        crate::rsass(
            "a {b: length(c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
