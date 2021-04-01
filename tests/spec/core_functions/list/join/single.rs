//! Tests auto-converted from "sass-spec/spec/core_functions/list/join/single.hrx"

mod both {
    mod comma {
        #[test]
        fn first() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: join((1,), [2])}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1, 2;\
        \n}\
        \n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \n\
            \na {b: join([1], (2,))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: [1, 2];\
        \n}\
        \n"
            );
        }
    }
    mod space {
        #[test]
        fn first() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: join(with-separator(1, space), [2])}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1 2;\
        \n}\
        \n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: join([1], with-separator(2, space))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: [1 2];\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn undecided() {
        assert_eq!(
            crate::rsass(
                "a {b: join([1], [2])}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: [1 2];\
        \n}\
        \n"
        );
    }
}
mod first {
    #[test]
    fn comma() {
        assert_eq!(
            crate::rsass(
                "a {b: join((1,), 2 3 4)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1, 2, 3, 4;\
        \n}\
        \n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \na {b: join(with-separator(1, space), (2, 3, 4))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1 2 3 4;\
        \n}\
        \n"
        );
    }
    mod undecided {
        #[test]
        fn and_comma() {
            assert_eq!(
                crate::rsass(
                    "a {b: join([1], (2, 3, 4))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: [1, 2, 3, 4];\
        \n}\
        \n"
            );
        }
        #[test]
        fn and_space() {
            assert_eq!(
                crate::rsass(
                    "a {b: join([1], 2 3 4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: [1 2 3 4];\
        \n}\
        \n"
            );
        }
    }
}
mod non_list {
    #[test]
    fn both() {
        assert_eq!(
            crate::rsass(
                "a {b: join(c, d)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c d;\
        \n}\
        \n"
        );
    }
    mod first {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "a {b: join(c, (d, e))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c, d, e;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(join(c, d e))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c d e;\
        \n}\
        \n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
        crate::rsass(
            "@import \"core_functions/list/utils\";\
            \n\
            \n$result: join(c, ());\
            \na {\
            \n  value: inspect($result);\
            \n  type: type-of($result);\
            \n\
            \n  // Note: LibSass\'s output here is strange but not strictly-speaking wrong.\
            \n  // See sass/libsass#2926 for details.\
            \n  separator: real-separator($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: c;\
        \n  type: list;\
        \n  separator: space;\
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
                    "a {b: join((c, d), e)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c, d, e;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(join(c d, e))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c d e;\
        \n}\
        \n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
        crate::rsass(
            "@import \"core_functions/list/utils\";\
            \n\
            \n$result: join((), c);\
            \na {\
            \n  value: inspect($result);\
            \n  type: type-of($result);\
            \n\
            \n  // Note: LibSass\'s output here is strange but not strictly-speaking wrong.\
            \n  // See sass/libsass#2926 for details.\
            \n  separator: real-separator($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: c;\
        \n  type: list;\
        \n  separator: space;\
        \n}\
        \n"
    );
        }
    }
}
mod second {
    #[test]
    fn comma() {
        assert_eq!(
            crate::rsass(
                "a {b: join(1 2 3, (4,))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1 2 3 4;\
        \n}\
        \n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \na {b: join((1, 2, 3), with-separator(4, space))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1, 2, 3, 4;\
        \n}\
        \n"
        );
    }
    mod undecided {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "a {b: join((1, 2, 3), [4])}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1, 2, 3, 4;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "a {b: join(1 2 3, [4])}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1 2 3 4;\
        \n}\
        \n"
            );
        }
    }
}
