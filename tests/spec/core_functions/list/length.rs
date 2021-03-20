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

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
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
