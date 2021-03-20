//! Tests auto-converted from "sass-spec/spec/values/numbers/modulo/floats.hrx"

mod larger {
    #[test]
    fn negative_negative() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  b: -6.3 % -2.4;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -1.5;\
        \n}\
        \n"
        );
    }
    #[test]
    fn negative_positive() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  b: -6.3 % 2.4;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.9;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive_negative() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  b: 6.3 % -2.4;\
            \n}"
            )
            .unwrap(),
            "a {\
        \n  b: -0.9;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive_positive() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  b: 6.3 % 2.4;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1.5;\
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
            \n  b: -1.2 % -4.7;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -1.2;\
        \n}\
        \n"
    );
}
#[test]
fn negative_positive() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: -1.2 % 4.7;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3.5;\
        \n}\
        \n"
    );
}
#[test]
fn positive_negative() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: 1.2 % -4.7;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -3.5;\
        \n}\
        \n"
    );
}
#[test]
fn positive_positive() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: 1.2 % 4.7;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1.2;\
        \n}\
        \n"
    );
}
