//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/three_args/percents.hrx"

mod all {
    #[test]
    fn percent() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(7.1%, 20.4%, 33.9%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #123456;\
        \n}\
        \n"
        );
    }
}
#[test]
fn boundaries() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(0%, 100%, 50%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #00ff80;\
        \n}\
        \n"
    );
}
mod clamped {
    #[test]
    fn blue() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 200%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: blue;\
        \n}\
        \n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, -0.1%, 0)}\
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
    fn red() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(100.1%, 0, 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: red;\
        \n}\
        \n"
        );
    }
}
mod percent {
    #[test]
    fn green() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(190, 68%, 237)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #beaded;\
        \n}\
        \n"
        );
    }
}
mod unitless {
    #[test]
    fn green() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(74.7%, 173, 93%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #beaded;\
        \n}\
        \n"
        );
    }
}
