//! Tests auto-converted from "sass-spec/spec/core_functions/color/grayscale.hrx"

#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: grayscale(rgba(#633736, 0.3))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(77, 77, 77, 0.3);\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn max_saturation() {
    assert_eq!(
        crate::rsass(
            "a {b: grayscale(red)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: gray;\
        \n}\
        \n"
    );
}
#[test]
fn mid_saturation() {
    assert_eq!(
        crate::rsass(
            "a {b: grayscale(#633736)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #4d4d4d;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: grayscale($color: white)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: white;\
        \n}\
        \n"
    );
}
mod no_saturation {
    #[test]
    fn black() {
        assert_eq!(
            crate::rsass(
                "a {b: grayscale(black)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: black;\
        \n}\
        \n"
        );
    }
    #[test]
    fn gray() {
        assert_eq!(
            crate::rsass(
                "a {b: grayscale(#494949)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #494949;\
        \n}\
        \n"
        );
    }
    #[test]
    fn white() {
        assert_eq!(
            crate::rsass(
                "a {b: grayscale(white)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: white;\
        \n}\
        \n"
        );
    }
}
#[test]
fn number() {
    assert_eq!(
        crate::rsass(
            "// A number should produce a plain function string, for CSS filter functions.\
            \na {b: grayscale(15%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: grayscale(15%);\
        \n}\
        \n"
    );
}
