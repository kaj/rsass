//! Tests auto-converted from "sass-spec/spec/core_functions/meta/type_of.hrx"

#[test]
#[ignore] // wrong result
fn arglist() {
    assert_eq!(
        crate::rsass(
            "@function type-of-arglist($args...) {\
            \n  @return type-of($args);\
            \n}\
            \n\
            \na {b: type-of-arglist()}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: arglist;\
        \n}\
        \n"
    );
}
mod boolean {
    #[test]
    fn test_false() {
        assert_eq!(
            crate::rsass(
                "a {b: type-of(false)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: bool;\
        \n}\
        \n"
        );
    }
    #[test]
    fn test_true() {
        assert_eq!(
            crate::rsass(
                "a {b: type-of(true)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: bool;\
        \n}\
        \n"
        );
    }
}
#[test]
fn color() {
    assert_eq!(
        crate::rsass(
            "a {b: type-of(red)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: color;\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
}
#[test]
fn function() {
    assert_eq!(
        crate::rsass(
            "a {b: type-of(get-function(\"type-of\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: function;\
        \n}\
        \n"
    );
}
mod list {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {b: type-of(())}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: list;\
        \n}\
        \n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            crate::rsass(
                "a {b: type-of(1 2 3)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: list;\
        \n}\
        \n"
        );
    }
}
mod map {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {b: type-of(map-remove((c: d), c))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: map;\
        \n}\
        \n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            crate::rsass(
                "a {b: type-of((c: d))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: map;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: type-of($value: c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: string;\
        \n}\
        \n"
    );
}
#[test]
fn null() {
    assert_eq!(
        crate::rsass(
            "a {b: type-of(null)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: null;\
        \n}\
        \n"
    );
}
mod number {
    #[test]
    fn unit() {
        assert_eq!(
            crate::rsass(
                "a {b: type-of(1.5px * 3.4em)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: number;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: type-of(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: number;\
        \n}\
        \n"
        );
    }
}
mod string {
    #[test]
    fn quoted() {
        assert_eq!(
            crate::rsass(
                "a {b: type-of(\"c\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: string;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            crate::rsass(
                "a {b: type-of(c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: string;\
        \n}\
        \n"
        );
    }
}
