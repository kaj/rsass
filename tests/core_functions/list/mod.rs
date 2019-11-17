//! Tests auto-converted from "sass-spec/spec/core_functions/list"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/list/append.hrx"
mod append {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn auto() {
        assert_eq!(
            rsass(
                "a {b: append(c d, e, $separator: auto)}\
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
    fn bracketed() {
        assert_eq!(
            rsass(
                "a {b: append([], 1)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: [1];\
             \n}\
             \n"
        );
    }
    mod comma {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn default() {
            assert_eq!(
                rsass(
                    "a {b: append((1, 2, 3), 4)}\
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
        fn overridden() {
            assert_eq!(
                rsass(
                    "a {b: append(1 2 3, 4, $separator: comma)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 1, 2, 3, 4;\
                 \n}\
                 \n"
            );
        }
    }
    mod empty {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn comma() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \n\
                     \n$result: append($empty-comma-list, 1);\
                     \na {\
                     \n  value: $result;\
                     \n  type: type-of($result);\
                     \n  separator: real-separator($result);\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  value: 1;\
                 \n  type: list;\
                 \n  separator: comma;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn space() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \n\
                     \n$result: append($empty-space-list, 1);\
                     \na {\
                     \n  value: $result;\
                     \n  type: type-of($result);\
                     \n  separator: real-separator($result);\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  value: 1;\
                 \n  type: list;\
                 \n  separator: space;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn undecided() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \n\
                     \n$result: append((), 1);\
                     \na {\
                     \n  value: $result;\
                     \n  type: type-of($result);\
                     \n  separator: real-separator($result);\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  value: 1;\
                 \n  type: list;\
                 \n  separator: space;\
                 \n}\
                 \n"
            );
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "separator", error tests are not supported yet.
        }

        // Ignoring "unknown_separator", error tests are not supported yet.
    }
    mod map {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn empty() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \n\
                     \n$result: append($empty-map, 1);\
                     \na {\
                     \n  value: $result;\
                     \n  type: type-of($result);\
                     \n  separator: real-separator($result);\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  value: 1;\
                 \n  type: list;\
                 \n  separator: space;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: append((c: d, e: f), g)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: c d, e f, g;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: append($list: c d, $val: e, $separator: comma)}\
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
    fn non_list() {
        assert_eq!(
            rsass(
                "a {b: append(c, d)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: c d;\
             \n}\
             \n"
        );
    }
    mod single {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn comma() {
            assert_eq!(
                rsass(
                    "a {b: append((1,), 2)}\
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
        #[ignore] // wrong result
        fn space() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \na {b: append(with-separator(1, space), 2)}\
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
        fn undecided() {
            assert_eq!(
                rsass(
                    "a {b: append(1, 2)}\
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
    mod space {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn default() {
            assert_eq!(
                rsass(
                    "a {b: append(1 2 3, 4)}\
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
        fn overridden() {
            assert_eq!(
                rsass(
                    "a {b: append((1, 2, 3), 4, $separator: space)}\
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

// From "sass-spec/spec/core_functions/list/index.hrx"
mod index {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
    }
    mod found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: index(a b c, a)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 1;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                rsass(
                    "a {b: index(a b c, c)}\
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
        fn map() {
            assert_eq!(
                rsass(
                    "a {b: index((c: d, e: f, g: h), e f)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 2;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn multiple() {
            assert_eq!(
                rsass(
                    "a {b: index(a b c a b c, b)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 2;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn non_list() {
            assert_eq!(
                rsass(
                    "a {b: index(c, c)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 1;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn sass_equality() {
            assert_eq!(
                rsass(
                    "a {b: index(1px 1in 1cm, 96px)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 2;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass(
                    "a {b: index([c], c)}\
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
    fn named() {
        assert_eq!(
            rsass(
                "a {b: index($list: c d e, $value: d)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 2;\
             \n}\
             \n"
        );
    }
    mod not_found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(index((), c))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: null;\
                 \n}\
                 \n"
            );
        }
        mod map {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn empty() {
                assert_eq!(
                    rsass(
                        "@import \"core_functions/list/utils\";\
                         \na {b: inspect(index($empty-map, e))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: null;\
                     \n}\
                     \n"
                );
            }
            #[test]
            fn non_empty() {
                assert_eq!(
                    rsass(
                        "a {b: inspect(index((c: d, e: f, g: h), e))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: null;\
                     \n}\
                     \n"
                );
            }
        }
        #[test]
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(index(c d e, f))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: null;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn non_list() {
            assert_eq!(
                rsass(
                    "a {b: inspect(index(c, d))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: null;\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/list/is_bracketed.hrx"
mod is_bracketed {
    #[allow(unused)]
    use super::rsass;
    mod bracketed {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: is-bracketed([])}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn multi() {
            assert_eq!(
                rsass(
                    "a {b: is-bracketed([1, 2, 3])}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass(
                    "a {b: is-bracketed([1])}\
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
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
    }
    mod unbracketed {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: is-bracketed(())}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn map() {
            assert_eq!(
                rsass(
                    "a {b: is-bracketed((c: d, e: f, g: h))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn multi() {
            assert_eq!(
                rsass(
                    "a {b: is-bracketed(1 2 3)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn non_list() {
            assert_eq!(
                rsass(
                    "a {b: is-bracketed(1)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass(
                    "a {b: is-bracketed((1,))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
    }
}

mod join;

// From "sass-spec/spec/core_functions/list/length.hrx"
mod length {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn t0() {
        assert_eq!(
            rsass(
                "a {b: length(())}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 0;\
             \n}\
             \n"
        );
    }
    #[test]
    fn t1() {
        assert_eq!(
            rsass(
                "a {b: length(join((), 1))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 1;\
             \n}\
             \n"
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            rsass(
                "a {b: length(c d)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 2;\
             \n}\
             \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
    }
    #[test]
    fn many() {
        assert_eq!(
            rsass(
                "a {b: length((1, 2, 3, 4, 5))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 5;\
             \n}\
             \n"
        );
    }
    mod map {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn empty() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \na {b: length($empty-map)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 0;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: length((1: 2, 3: 4))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 2;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: length($list: 1 2 3)}\
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
    fn non_list() {
        assert_eq!(
            rsass(
                "a {b: length(c)}\
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

// From "sass-spec/spec/core_functions/list/nth.hrx"
mod nth {
    #[allow(unused)]
    use super::rsass;
    mod t1 {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn of_1() {
            assert_eq!(
                rsass(
                    "a {b: nth(join((), c), 1)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: c;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn of_2() {
            assert_eq!(
                rsass(
                    "a {b: nth(c d, 1)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: c;\
                 \n}\
                 \n"
            );
        }
    }
    mod t2 {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn of_2() {
            assert_eq!(
                rsass(
                    "a {b: nth(c d, 2)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: d;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn of_4() {
            assert_eq!(
                rsass(
                    "a {b: nth(c d e f, 2)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: d;\
                 \n}\
                 \n"
            );
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod index {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "t0", error tests are not supported yet.

            // Ignoring "too_high", error tests are not supported yet.

            // Ignoring "too_low", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn map() {
        assert_eq!(
            rsass(
                "a {b: nth((c: d, e: f, g: h), 2)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: e f;\
             \n}\
             \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: nth($list: c d, $n: 1)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: c;\
             \n}\
             \n"
        );
    }
    mod negative {
        #[allow(unused)]
        use super::rsass;
        mod t1 {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn of_1() {
                assert_eq!(
                    rsass(
                        "a {b: nth(join((), c), -1)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: c;\
                     \n}\
                     \n"
                );
            }
            #[test]
            fn of_2() {
                assert_eq!(
                    rsass(
                        "a {b: nth(c d, -1)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: d;\
                     \n}\
                     \n"
                );
            }
        }
        mod t2 {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn of_2() {
                assert_eq!(
                    rsass(
                        "a {b: nth(c d, -2)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: c;\
                     \n}\
                     \n"
                );
            }
            #[test]
            fn of_4() {
                assert_eq!(
                    rsass(
                        "a {b: nth(c d e f, -2)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: e;\
                     \n}\
                     \n"
                );
            }
        }
    }
    #[test]
    fn non_list() {
        assert_eq!(
            rsass(
                "a {b: nth(c, 1)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: c;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/list/separator.hrx"
mod separator {
    #[allow(unused)]
    use super::rsass;
    mod empty {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn comma() {
            assert_eq!(
                rsass(
                    "a {b: list-separator(join((), (), comma))}\
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
        #[ignore] // unexepected error
        fn map() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \n\
                     \na {b: list-separator($empty-map)}\
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
        fn space() {
            assert_eq!(
                rsass(
                    "a {b: list-separator(())}\
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
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
    }
    mod multi {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn comma() {
            assert_eq!(
                rsass(
                    "a {b: list-separator((1, 2, 3))}\
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
        fn map() {
            assert_eq!(
                rsass(
                    "a {b: list-separator((c: d, e: f, g: h))}\
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
                rsass(
                    "a {b: list-separator(1 2 3)}\
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn comma() {
            assert_eq!(
                rsass(
                    "a {b: list-separator((1,))}\
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
        fn non_list() {
            assert_eq!(
                rsass(
                    "a {b: list-separator(1)}\
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
        fn space() {
            assert_eq!(
                rsass(
                    "a {b: list-separator([1])}\
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

// From "sass-spec/spec/core_functions/list/set_nth.hrx"
mod set_nth {
    #[allow(unused)]
    use super::rsass;
    mod t1 {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn of_1() {
            assert_eq!(
                rsass(
                    "$result: set-nth(join((), b), 1, c);\
                     \na {\
                     \n  result: $result;\
                     \n  type: type-of($result);\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  result: c;\
                 \n  type: list;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn of_2() {
            assert_eq!(
                rsass(
                    "a {b: set-nth(c d, 1, e)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: e d;\
                 \n}\
                 \n"
            );
        }
    }
    mod t2 {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn of_2() {
            assert_eq!(
                rsass(
                    "a {b: set-nth(c d, 2, e)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: c e;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn of_4() {
            assert_eq!(
                rsass(
                    "a {b: set-nth(c d e f, 2, g)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: c g e f;\
                 \n}\
                 \n"
            );
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod index {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "t0", error tests are not supported yet.

            // Ignoring "too_few_args", error tests are not supported yet.

            // Ignoring "too_high", error tests are not supported yet.

            // Ignoring "too_low", error tests are not supported yet.

            // Ignoring "too_many_args", error tests are not supported yet.
        }

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn map() {
        assert_eq!(
            rsass(
                "a {b: set-nth((c: d, e: f, g: h), 2, i)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: c d, i, g h;\
             \n}\
             \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: set-nth($list: c d, $n: 1, $value: e)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: e d;\
             \n}\
             \n"
        );
    }
    mod negative {
        #[allow(unused)]
        use super::rsass;
        mod t1 {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn of_1() {
                assert_eq!(
                    rsass(
                        "$result: set-nth(join((), b), -1, c);\
                         \na {\
                         \n  result: $result;\
                         \n  type: type-of($result);\
                         \n}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  result: c;\
                     \n  type: list;\
                     \n}\
                     \n"
                );
            }
            #[test]
            fn of_2() {
                assert_eq!(
                    rsass(
                        "a {b: set-nth(c d, -1, e)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: c e;\
                     \n}\
                     \n"
                );
            }
        }
        mod t2 {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn of_2() {
                assert_eq!(
                    rsass(
                        "a {b: set-nth(c d, -2, e)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: e d;\
                     \n}\
                     \n"
                );
            }
            #[test]
            fn of_4() {
                assert_eq!(
                    rsass(
                        "a {b: set-nth(c d e f, -2, g)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: c d g f;\
                     \n}\
                     \n"
                );
            }
        }
    }
    #[test]
    fn non_list() {
        assert_eq!(
            rsass(
                "$result: set-nth(b, 1, c);\
                 \na {\
                 \n  result: $result;\
                 \n  type: type-of($result);\
                 \n}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  result: c;\
             \n  type: list;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/list/utils.hrx"
mod utils {
    #[allow(unused)]
    use super::rsass;
    mod empty_map {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn same_as_empty_list() {
            assert_eq!(
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        mod empty {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn comma() {
                assert_eq!(
                    rsass(
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
            #[ignore] // unexepected error
            fn space() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn undecided() {
                assert_eq!(
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn comma() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn space() {
                assert_eq!(
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn comma() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn undecided() {
                assert_eq!(
                    rsass(
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
        #[allow(unused)]
        use super::rsass;
        mod multi {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn comma() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn space() {
                assert_eq!(
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn comma() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn space() {
                assert_eq!(
                    rsass(
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
}

// From "sass-spec/spec/core_functions/list/zip.hrx"
mod zip {
    #[allow(unused)]
    use super::rsass;
    mod map {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(zip(map-remove((c: d), c)))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: ();\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(zip((c: d, e: f, g: h), 1 2 3))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: (c d) 1, (e f) 2, (g h) 3;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn no_lists() {
        assert_eq!(
            rsass(
                "@import \"core_functions/list/utils\";\
                 \n\
                 \n$result: zip();\
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
    #[ignore] // wrong result
    fn non_list() {
        assert_eq!(
            rsass(
                "a {b: zip(c, d, e)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: c d e;\
             \n}\
             \n"
        );
    }
    mod one_list {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn bracketed() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \n\
                     \n$result: zip([1 2 3]);\
                     \n$element: nth($result, 2);\
                     \n\
                     \na {\
                     \n  value: $result;\
                     \n  element: $element {\
                     \n    type: type-of($element);\
                     \n    separator: real-separator($element);\
                     \n  }\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  value: 1, 2, 3;\
                 \n  element: 2;\
                 \n  element-type: list;\
                 \n  element-separator: space;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn comma() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \n\
                     \n$result: zip((1, 2, 3));\
                     \n$element: nth($result, 2);\
                     \n\
                     \na {\
                     \n  value: $result;\
                     \n  element: $element {\
                     \n    type: type-of($element);\
                     \n    separator: real-separator($element);\
                     \n  }\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  value: 1, 2, 3;\
                 \n  element: 2;\
                 \n  element-type: list;\
                 \n  element-separator: space;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn empty() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \n\
                     \n$result: zip(());\
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
        #[ignore] // unexepected error
        fn space() {
            assert_eq!(
                rsass(
                    "@import \"core_functions/list/utils\";\
                     \n\
                     \n$result: zip(1 2 3);\
                     \n$element: nth($result, 2);\
                     \n\
                     \na {\
                     \n  value: $result;\
                     \n  element: $element {\
                     \n    type: type-of($element);\
                     \n    separator: real-separator($element);\
                     \n  }\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  value: 1, 2, 3;\
                 \n  element: 2;\
                 \n  element-type: list;\
                 \n  element-separator: space;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    fn three_lists() {
        assert_eq!(
            rsass(
                "a {b: zip(1 2 3, c d e, red green blue)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 1 c red, 2 d green, 3 e blue;\
             \n}\
             \n"
        );
    }
    mod two_lists {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn first_empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(zip((), 1 2 3))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: ();\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn first_longer() {
            assert_eq!(
                rsass(
                    "a {b: zip(1 2 3 4, c d)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 1 c, 2 d;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn same_length() {
            assert_eq!(
                rsass(
                    "a {b: zip(1 2 3, c d e)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 1 c, 2 d, 3 e;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn second_empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(zip(1 2 3, ()))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: ();\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn second_longer() {
            assert_eq!(
                rsass(
                    "a {b: zip(1 2, c d e f)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 1 c, 2 d;\
                 \n}\
                 \n"
            );
        }
    }
}
