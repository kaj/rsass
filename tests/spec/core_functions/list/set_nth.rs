//! Tests auto-converted from "sass-spec/spec/core_functions/list/set_nth.hrx"

mod t1 {
    #[test]
    fn of_1() {
        assert_eq!(
            crate::rsass(
                "$result: set-nth(join((), b), 1, c);\
            \na {\
            \n  result: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  result: c;\
        \n  type: list;\
        \n}\
        \n"
        );
    }
    #[test]
    fn of_2() {
        assert_eq!(
            crate::rsass(
                "a {b: set-nth(c d, 1, e)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: e d;\
        \n}\
        \n"
        );
    }
}
mod t2 {
    #[test]
    fn of_2() {
        assert_eq!(
            crate::rsass(
                "a {b: set-nth(c d, 2, e)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c e;\
        \n}\
        \n"
        );
    }
    #[test]
    fn of_4() {
        assert_eq!(
            crate::rsass(
                "a {b: set-nth(c d e f, 2, g)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c g e f;\
        \n}\
        \n"
        );
    }
}
mod error {
    mod index {

        // Ignoring "t0", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_high", error tests are not supported yet.

        // Ignoring "too_low", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
    }

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn map() {
    assert_eq!(
        crate::rsass(
            "a {b: set-nth((c: d, e: f, g: h), 2, i)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c d, i, g h;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: set-nth($list: c d, $n: 1, $value: e)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: e d;\
        \n}\
        \n"
    );
}
mod negative {
    mod t1 {
        #[test]
        fn of_1() {
            assert_eq!(
                crate::rsass(
                    "$result: set-nth(join((), b), -1, c);\
            \na {\
            \n  result: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  result: c;\
        \n  type: list;\
        \n}\
        \n"
            );
        }
        #[test]
        fn of_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: set-nth(c d, -1, e)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c e;\
        \n}\
        \n"
            );
        }
    }
    mod t2 {
        #[test]
        fn of_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: set-nth(c d, -2, e)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: e d;\
        \n}\
        \n"
            );
        }
        #[test]
        fn of_4() {
            assert_eq!(
                crate::rsass(
                    "a {b: set-nth(c d e f, -2, g)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c d g f;\
        \n}\
        \n"
            );
        }
    }
}
#[test]
fn non_list() {
    assert_eq!(
        crate::rsass(
            "$result: set-nth(b, 1, c);\
            \na {\
            \n  result: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  result: c;\
        \n  type: list;\
        \n}\
        \n"
    );
}
