//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/two_args.hrx"

mod clamped {
    #[test]
    fn opaque() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(#123, 1.1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #112233;\
        \n}\
        \n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(#123, -0.1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(17, 34, 51, 0);\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: rgba($color: #123, $alpha: 0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(17, 34, 51, 0.5);\
        \n}\
        \n"
    );
}
mod opaque_to {
    #[test]
    fn opaque() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(#123, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #112233;\
        \n}\
        \n"
        );
    }
    #[test]
    fn partial() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(#123, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(17, 34, 51, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(#123, 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(17, 34, 51, 0);\
        \n}\
        \n"
        );
    }
}
mod partial_to {
    #[test]
    fn opaque() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(rgba(0, 0, 255, 0.3), 1)}\
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
    fn partial() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(rgba(0, 0, 255, 0.3), 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 255, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(rgba(0, 0, 255, 0.3), 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 255, 0);\
        \n}\
        \n"
        );
    }
}
mod transparent_to {
    #[test]
    fn opaque() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(transparent, 1)}\
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
    fn partial() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(transparent, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(transparent, 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0);\
        \n}\
        \n"
        );
    }
}
