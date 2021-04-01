//! Tests auto-converted from "sass-spec/spec/core_functions/list/utils.hrx"

mod empty_map {
    #[test]
    fn same_as_empty_list() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \na {b: $empty-map == ()}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
}
mod real_separator {
    mod empty {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: real-separator($empty-comma-list)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: comma;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: real-separator($empty-space-list)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: space;\
        \n}\
        \n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: real-separator(())}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: undecided;\
        \n}\
        \n"
            );
        }
    }
    mod multi {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: real-separator((1, 2))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: comma;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: real-separator(1 2)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: space;\
        \n}\
        \n"
            );
        }
    }
    mod single {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: real-separator((1,))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: comma;\
        \n}\
        \n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: real-separator([1])}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: undecided;\
        \n}\
        \n"
            );
        }
    }
}
mod with_separator {
    mod multi {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: with-separator(1 2, comma)}\
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
        fn space() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: with-separator((1, 2), space)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1 2;\
        \n}\
        \n"
            );
        }
    }
    mod single {
        #[test]
        fn comma() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: real-separator(with-separator([1], comma))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: comma;\
        \n}\
        \n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: real-separator(with-separator((1,), space))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: space;\
        \n}\
        \n"
            );
        }
    }
}
