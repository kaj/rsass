//! Tests auto-converted from "sass-spec/spec/core_functions/list/join/multi.hrx"

mod auto {
    #[test]
    fn bracketed() {
        assert_eq!(
            crate::rsass(
                "a {b: join(c d, e f, $bracketed: auto)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c d e f;\
        \n}\
        \n"
        );
    }
    #[test]
    fn separator() {
        assert_eq!(
            crate::rsass(
                "a {b: join((c, d), e f, $separator: auto)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c, d, e, f;\
        \n}\
        \n"
        );
    }
}
mod bracketed {
    #[test]
    fn and_separator() {
        assert_eq!(
            crate::rsass(
                "a {b: join(c, d, $bracketed: true, $separator: comma)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: [c, d];\
        \n}\
        \n"
        );
    }
    #[test]
    fn both() {
        assert_eq!(
            crate::rsass(
                "a {b: join([c d], [e f])}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: [c d e f];\
        \n}\
        \n"
        );
    }
    #[test]
    fn test_false() {
        assert_eq!(
            crate::rsass(
                "a {b: join([c], [d], $bracketed: false)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c d;\
        \n}\
        \n"
        );
    }
    #[test]
    fn falsey() {
        assert_eq!(
            crate::rsass(
                "a {b: join([c], [d], $bracketed: null)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c d;\
        \n}\
        \n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            crate::rsass(
                "a {b: join([c d], e f)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: [c d e f];\
        \n}\
        \n"
        );
    }
    #[test]
    fn positional() {
        assert_eq!(
            crate::rsass(
                "a {b: join(c, d, comma, true)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: [c, d];\
        \n}\
        \n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            crate::rsass(
                "a {b: join(c d, [e f])}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c d e f;\
        \n}\
        \n"
        );
    }
    #[test]
    fn test_true() {
        assert_eq!(
            crate::rsass(
                "a {b: join(c, d, $bracketed: true)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: [c d];\
        \n}\
        \n"
        );
    }
    #[test]
    fn truthy() {
        assert_eq!(
            crate::rsass(
                "a {b: join(c, d, $bracketed: e)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: [c d];\
        \n}\
        \n"
        );
    }
}
mod comma {
    #[test]
    fn both() {
        assert_eq!(
            crate::rsass(
                "a {b: join((c, d), (e, f))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c, d, e, f;\
        \n}\
        \n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            crate::rsass(
                "a {b: join((c, d), e f)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c, d, e, f;\
        \n}\
        \n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            crate::rsass(
                "a {b: join(c d, (e, f))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c d e f;\
        \n}\
        \n"
        );
    }
    mod separator {
        #[test]
        fn forces_comma() {
            assert_eq!(
                crate::rsass(
                    "a {b: join(c, d, $separator: comma)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c, d;\
        \n}\
        \n"
            );
        }
        #[test]
        fn forces_not_comma() {
            assert_eq!(
                crate::rsass(
                    "a {b: join((c, d), (e, f), $separator: space)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c d e f;\
        \n}\
        \n"
            );
        }
    }
}
mod map {
    #[test]
    fn both() {
        assert_eq!(
            crate::rsass(
                "a {b: join((c: d, e: f), (g: h, i: j))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c d, e f, g h, i j;\
        \n}\
        \n"
        );
    }
    mod first {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "a {b: join((c: d, e: f), (g, h))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c d, e f, g, h;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(join((c: d, e: f), g h))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c d, e f, g, h;\
        \n}\
        \n"
            );
        }
    }
    mod second {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "a {b: join((c, d), (e: f, g: h))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c, d, e f, g h;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
        crate::rsass(
            "// Use inspect() to prove that the map is converted to a list of pairs.\
            \na {b: inspect(join(c d, (e: f, g: h)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c d (e f) (g h);\
        \n}\
        \n"
    );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: join($list1: a b, $list2: c d, $separator: comma, $bracketed: true)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: [a, b, c, d];\
        \n}\
        \n"
    );
}
mod space {
    #[test]
    fn both() {
        assert_eq!(
            crate::rsass(
                "a {b: join(c d, e f)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c d e f;\
        \n}\
        \n"
        );
    }
    mod separator {
        #[test]
        fn forces_not_space() {
            assert_eq!(
                crate::rsass(
                    "a {b: join(c d, e f, $separator: comma)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c, d, e, f;\
        \n}\
        \n"
            );
        }
        #[test]
        fn forces_space() {
            assert_eq!(
                crate::rsass(
                    "a {b: join(c, d, $separator: space)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c d;\
        \n}\
        \n"
            );
        }
    }
}
