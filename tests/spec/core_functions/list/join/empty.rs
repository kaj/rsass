//! Tests auto-converted from "sass-spec/spec/core_functions/list/join/empty.hrx"

mod both {
    mod comma {
        #[test]
        fn first() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \n\
            \n$result: join($empty-comma-list, ());\
            \na {\
            \n  value: inspect($result);\
            \n  separator: real-separator($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: ();\
        \n  separator: comma;\
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
            \n$result: join((), $empty-comma-list);\
            \na {\
            \n  value: inspect($result);\
            \n  separator: real-separator($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: ();\
        \n  separator: comma;\
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
            \n\
            \n$result: join($empty-space-list, ());\
            \na {\
            \n  value: inspect($result);\
            \n  separator: real-separator($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: ();\
        \n  separator: space;\
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
            \n$result: join((), $empty-space-list);\
            \na {\
            \n  value: inspect($result);\
            \n  separator: real-separator($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: ();\
        \n  separator: space;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn undecided() {
        assert_eq!(
        crate::rsass(
            "@import \"core_functions/list/utils\";\
            \n\
            \n$result: join((), ());\
            \na {\
            \n  value: inspect($result);\
            \n\
            \n  // `join()` should always produce a real separator, even when the inputs have\
            \n  // undecided separators. It should default to `space`.\
            \n  separator: real-separator($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: ();\
        \n  separator: space;\
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
                "@import \"core_functions/list/utils\";\
            \na {b: join($empty-comma-list, 1 2 3)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1, 2, 3;\
        \n}\
        \n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \na {b: join($empty-space-list, (1, 2, 3))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1 2 3;\
        \n}\
        \n"
        );
    }
    mod undecided {
        #[test]
        fn and_comma() {
            assert_eq!(
                crate::rsass(
                    "a {b: join((), (1, 2, 3))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1, 2, 3;\
        \n}\
        \n"
            );
        }
        #[test]
        fn and_space() {
            assert_eq!(
                crate::rsass(
                    "a {b: join((), 1 2 3)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1 2 3;\
        \n}\
        \n"
            );
        }
    }
}
mod map {
    mod first {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: join($empty-map, (1, 2, 3))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1, 2, 3;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: join($empty-map, 1 2 3)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1 2 3;\
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
            \n$result: join($empty-map, ());\
            \na {\
            \n  value: inspect($result);\
            \n  separator: real-separator($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: ();\
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
                    "@import \"core_functions/list/utils\";\
            \na {b: join((1, 2, 3), $empty-map)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1, 2, 3;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: join(1 2 3, $empty-map)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1 2 3;\
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
            \n$result: join($empty-map, ());\
            \na {\
            \n  value: inspect($result);\
            \n  separator: real-separator($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: ();\
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
                "@import \"core_functions/list/utils\";\
            \na {b: join(1 2 3, $empty-comma-list)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1 2 3;\
        \n}\
        \n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \na {b: join((1, 2, 3), $empty-space-list)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1, 2, 3;\
        \n}\
        \n"
        );
    }
    mod undecided {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "a {b: join((1, 2, 3), ())}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1, 2, 3;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "a {b: join(1 2 3, ())}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1 2 3;\
        \n}\
        \n"
            );
        }
    }
}
