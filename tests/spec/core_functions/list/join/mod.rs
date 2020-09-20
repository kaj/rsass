//! Tests auto-converted from "sass-spec/spec/core_functions/list/join"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/list/join/empty.hrx"
mod empty {
    #[allow(unused)]
    use super::rsass;
    mod both {
        #[allow(unused)]
        use super::rsass;
        mod comma {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn first() {
                assert_eq!(
                    rsass(
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
            #[ignore] // unexepected error
            fn last() {
                assert_eq!(
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn first() {
                assert_eq!(
                    rsass(
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
            #[ignore] // unexepected error
            fn last() {
                assert_eq!(
                    rsass(
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
        #[ignore] // wrong result
        fn undecided() {
            assert_eq!(
        rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn comma() {
            assert_eq!(
                rsass(
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
        #[ignore] // unexepected error
        fn space() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn and_comma() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
        #[allow(unused)]
        use super::rsass;
        mod first {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn comma() {
                assert_eq!(
                    rsass(
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
            #[ignore] // unexepected error
            fn space() {
                assert_eq!(
                    rsass(
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
            #[ignore] // unexepected error
            fn undecided() {
                assert_eq!(
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn comma() {
                assert_eq!(
                    rsass(
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
            #[ignore] // unexepected error
            fn space() {
                assert_eq!(
                    rsass(
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
            #[ignore] // unexepected error
            fn undecided() {
                assert_eq!(
                    rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn comma() {
            assert_eq!(
                rsass(
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
        #[ignore] // unexepected error
        fn space() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn comma() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
}

// From "sass-spec/spec/core_functions/list/join/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;

    // Ignoring "named", error tests are not supported yet.

    // Ignoring "positional_and_named", error tests are not supported yet.

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "separator", error tests are not supported yet.
    }

    // Ignoring "unknown_separator", error tests are not supported yet.
}

// From "sass-spec/spec/core_functions/list/join/multi.hrx"
mod multi {
    #[allow(unused)]
    use super::rsass;
    mod auto {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn bracketed() {
            assert_eq!(
                rsass(
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
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn and_separator() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn both() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn forces_comma() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn both() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn comma() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn comma() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn space() {
                assert_eq!(
        rsass(
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
        rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn both() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn forces_not_space() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
}

// From "sass-spec/spec/core_functions/list/join/single.hrx"
mod single {
    #[allow(unused)]
    use super::rsass;
    mod both {
        #[allow(unused)]
        use super::rsass;
        mod comma {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn first() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn last() {
                assert_eq!(
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn first() {
                assert_eq!(
                    rsass(
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
            #[ignore] // wrong result
            fn last() {
                assert_eq!(
                    rsass(
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
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn comma() {
            assert_eq!(
                rsass(
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
        #[ignore] // wrong result
        fn space() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn and_comma() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn both() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn comma() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
            #[ignore] // wrong result
            fn undecided() {
                assert_eq!(
        rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn comma() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
            #[ignore] // wrong result
            fn undecided() {
                assert_eq!(
        rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn comma() {
            assert_eq!(
                rsass(
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
        #[ignore] // wrong result
        fn space() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn comma() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
}
