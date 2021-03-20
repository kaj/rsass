//! Tests auto-converted from "sass-spec/spec/core_functions/map/remove.hrx"

mod error {

    // Ignoring "positional_and_named", error tests are not supported yet.

    // Ignoring "too_few_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "map", error tests are not supported yet.
    }
}
mod found {
    #[test]
    fn first() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 1))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (3: 4, 5: 6);\
        \n}\
        \n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 5))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (1: 2, 3: 4);\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 3))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (1: 2, 5: 6);\
        \n}\
        \n"
        );
    }
    mod multiple {
        #[test]
        fn all() {
            assert_eq!(
        crate::rsass(
            "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6, 7: 8, 9: 10), 1, 5, 9))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (3: 4, 7: 8);\
        \n}\
        \n"
    );
        }
        #[test]
        fn some() {
            assert_eq!(
        crate::rsass(
            "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6, 7: 8), 1, 5, 9))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (3: 4, 7: 8);\
        \n}\
        \n"
    );
        }
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-remove((c: d), c))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ();\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: inspect(map-remove($map: (c: d), $key: c))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: ();\
        \n}\
        \n"
    );
}
mod not_found {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-remove((), 1))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ();\
        \n}\
        \n"
        );
    }
    #[test]
    fn multiple() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-remove((c: d), e, f, g))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d);\
        \n}\
        \n"
        );
    }
    #[test]
    fn no_keys() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-remove((c: d)))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d);\
        \n}\
        \n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-remove((c: d), d))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d);\
        \n}\
        \n"
        );
    }
}
