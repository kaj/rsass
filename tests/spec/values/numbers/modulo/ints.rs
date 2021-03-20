//! Tests auto-converted from "sass-spec/spec/values/numbers/modulo/ints.hrx"

mod larger {
    #[test]
    fn negative_negative() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  b: -7 % -5;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn negative_positive() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  b: -7 % 5;\
            \n}\
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
    fn positive_negative() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  b: 6 % -5;\
            \n}"
            )
            .unwrap(),
            "a {\
        \n  b: -4;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive_positive() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  b: 6 % 5;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
}
#[test]
fn negative_negative() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: -1 % -4;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -1;\
        \n}\
        \n"
    );
}
#[test]
fn negative_positive() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: -1 % 4;\
            \n}\
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
fn positive_negative() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: 1 % -4;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  b: -3;\
        \n}\
        \n"
    );
}
#[test]
fn positive_positive() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: 1 % 4;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
