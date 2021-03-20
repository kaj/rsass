//! Tests auto-converted from "sass-spec/spec/core_functions/list/nth.hrx"

mod t1 {
    #[test]
    fn of_1() {
        assert_eq!(
            crate::rsass(
                "a {b: nth(join((), c), 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
    #[test]
    fn of_2() {
        assert_eq!(
            crate::rsass(
                "a {b: nth(c d, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
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
                "a {b: nth(c d, 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: d;\
        \n}\
        \n"
        );
    }
    #[test]
    fn of_4() {
        assert_eq!(
            crate::rsass(
                "a {b: nth(c d e f, 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: d;\
        \n}\
        \n"
        );
    }
}
mod error {
    mod index {

        // Ignoring "t0", error tests are not supported yet.

        // Ignoring "too_high", error tests are not supported yet.

        // Ignoring "too_low", error tests are not supported yet.
    }

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn map() {
    assert_eq!(
        crate::rsass(
            "a {b: nth((c: d, e: f, g: h), 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: e f;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: nth($list: c d, $n: 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
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
                    "a {b: nth(join((), c), -1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c;\
        \n}\
        \n"
            );
        }
        #[test]
        fn of_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: nth(c d, -1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: d;\
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
                    "a {b: nth(c d, -2)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c;\
        \n}\
        \n"
            );
        }
        #[test]
        fn of_4() {
            assert_eq!(
                crate::rsass(
                    "a {b: nth(c d e f, -2)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: e;\
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
            "a {b: nth(c, 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
