//! Tests auto-converted from "sass-spec/spec/core_functions/map"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/map/deep_merge.hrx"
mod deep_merge {
    #[allow(unused)]
    use super::rsass;
    mod deep {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn different_keys() {
            assert_eq!(
        rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: (d: e, f: g)), (c: (1: 2, 3: 4))))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (1: 2, 3: 4, d: e, f: g));\
        \n}\
        \n"
    );
        }
        mod empty {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn first() {
                assert_eq!(
                    rsass(
                        "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: ()), (c: (d: e))))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: (d: e));\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn second() {
                assert_eq!(
                    rsass(
                        "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: (d: e)), (c: ())))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: (d: e));\
        \n}\
        \n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn multiple_layers() {
            assert_eq!(
        rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: (d: (e: (f: g)))), (c: (d: (e: (1: 2))))))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: (e: (1: 2, f: g))));\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn overlapping_keys() {
            assert_eq!(
        rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: (d: e, f: g, h: i)), (c: (j: 1, f: 2, k: 3))))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (j: 1, f: 2, k: 3, d: e, h: i));\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn same_keys() {
            assert_eq!(
        rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: (d: e, f: g)), (c: (d: 1, f: 2))))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: 1, f: 2));\
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

            // Ignoring "map1", error tests are not supported yet.

            // Ignoring "map2", error tests are not supported yet.
        }
    }
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
            rsass(
                "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge($map1: (c: d), $map2: (1: 2)))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (1: 2, c: d);\
        \n}\
        \n"
        );
    }
    mod shallow {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn different_keys() {
            assert_eq!(
                rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: d, e: f), (1: 2, 3: 4)))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (1: 2, 3: 4, c: d, e: f);\
        \n}\
        \n"
            );
        }
        mod empty {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn both() {
                assert_eq!(
                    rsass(
                        "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((), ()))}\
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
            fn first() {
                assert_eq!(
                    rsass(
                        "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((), (c: d, e: f)))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: d, e: f);\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn second() {
                assert_eq!(
                    rsass(
                        "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: d, e: f), ()))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: d, e: f);\
        \n}\
        \n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn overlapping_keys() {
            assert_eq!(
        rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: d, e: f, g: h), (i: 1, e: 2, j: 3)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (i: 1, e: 2, j: 3, c: d, g: h);\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn same_keys() {
            assert_eq!(
                rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: d, e: f), (c: 1, e: 2)))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: 1, e: 2);\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/map/deep_remove.hrx"
mod deep_remove {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    mod found {
        #[allow(unused)]
        use super::rsass;
        mod nested {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn first() {
                assert_eq!(
                    rsass(
                        "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e, f: g, h: i)), c, d))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: (f: g, h: i));\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn last() {
                assert_eq!(
                    rsass(
                        "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e, f: g, h: i)), c, h))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: (d: e, f: g));\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn middle() {
                assert_eq!(
                    rsass(
                        "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e, f: g, h: i)), c, f))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: (d: e, h: i));\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn single() {
                assert_eq!(
                    rsass(
                        "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e)), c, d))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: ());\
        \n}\
        \n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn top_level() {
            assert_eq!(
                rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: d), c))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: ();\
        \n}\
        \n"
            );
        }
    }
    mod not_found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn empty() {
            assert_eq!(
                rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((), 1))}\
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
        fn extra_keys() {
            assert_eq!(
                rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e)), c, d, e, f, g))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: e));\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn nested() {
            assert_eq!(
                rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e)), c, e))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: e));\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn not_a_map() {
            assert_eq!(
                rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e)), c, d, e))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: e));\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn top_level() {
            assert_eq!(
                rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: d), d))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: d);\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/map/get.hrx"
mod get {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "map", error tests are not supported yet.
        }
    }
    mod found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: map-get((1: 2, 3: 4, 5: 6), 1)}\
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
        fn last() {
            assert_eq!(
                rsass(
                    "a {b: map-get((1: 2, 3: 4, 5: 6), 5)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 6;\
        \n}\
        \n"
            );
        }
        #[test]
        fn middle() {
            assert_eq!(
                rsass(
                    "a {b: map-get((1: 2, 3: 4, 5: 6), 3)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 4;\
        \n}\
        \n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass(
                    "a {b: map-get((c: d), c)}\
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
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: map-get($map: (c: d), $key: c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: d;\
        \n}\
        \n"
        );
    }
    mod nested {
        #[allow(unused)]
        use super::rsass;
        mod found {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn full_path() {
                assert_eq!(
                    rsass(
                        "a {b: map-get((c: (d: (e: f))), c, d, e)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: f;\
        \n}\
        \n"
                );
            }
            #[test]
            fn partial_path() {
                assert_eq!(
                    rsass(
                        "a {b: inspect(map-get((c: (d: (e: f))), c, d))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (e: f);\
        \n}\
        \n"
                );
            }
        }
        mod not_found {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn deep() {
                assert_eq!(
                    rsass(
                        "a {b: inspect(map-get((c: (d: (e: f))), c, d, g))}\
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
            fn too_many_keys() {
                assert_eq!(
        rsass(
            "a {b: inspect(map-get((c: (d: (e: f))), c, d, e, f))}\
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
            fn top_level() {
                assert_eq!(
                    rsass(
                        "a {b: inspect(map-get((c: (d: (e: f))), d))}\
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
    mod not_found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn dash_sensitive() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-get((c-d: e), c_d))}\
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
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-get((), 1))}\
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
                    "a {b: inspect(map-get((c: d), d))}\
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

// From "sass-spec/spec/core_functions/map/has_key.hrx"
mod has_key {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "map", error tests are not supported yet.
        }
    }
    mod found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((1: 2, 3: 4, 5: 6), 1)}\
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
        fn last() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((1: 2, 3: 4, 5: 6), 5)}\
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
        fn middle() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((1: 2, 3: 4, 5: 6), 3)}\
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
                    "a {b: map-has-key((c: d), c)}\
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
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: map-has-key($map: (c: d), $key: c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
    mod nested {
        #[allow(unused)]
        use super::rsass;
        mod found {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn full_path() {
                assert_eq!(
                    rsass(
                        "a {b: map-has-key((c: (d: (e: f))), c, d, e)}\
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
            fn partial_path() {
                assert_eq!(
                    rsass(
                        "a {b: map-has-key((c: (d: (e: f))), c, d)}\
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
        mod not_found {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn deep() {
                assert_eq!(
                    rsass(
                        "a {b: map-has-key((c: (d: (e: f))), c, d, g)}\
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
            fn too_many_keys() {
                assert_eq!(
                    rsass(
                        "a {b: map-has-key((c: (d: (e: f))), c, d, e, f)}\
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
            fn top_level() {
                assert_eq!(
                    rsass(
                        "a {b: map-has-key((c: (d: (e: f))), d)}\
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
    mod not_found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((), 1)}\
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
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((c: d), d)}\
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

// From "sass-spec/spec/core_functions/map/keys.hrx"
mod keys {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn empty() {
        assert_eq!(
            rsass(
                "$result: map-keys(());\
            \na {\
            \n  value: inspect($result);\
            \n  separator: list-separator($result);\
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
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn multiple() {
        assert_eq!(
            rsass(
                "a {b: map-keys((c: d, e: f, g: h))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c, e, g;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: map-keys($map: (1: 2, 3: 4))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1, 3;\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            rsass(
                "$result: map-keys((1: 2));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n  separator: list-separator($result);\
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
}

// From "sass-spec/spec/core_functions/map/merge.hrx"
mod merge {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn different_keys() {
        assert_eq!(
            rsass(
                "a {b: inspect(map-merge((c: d, e: f), (1: 2, 3: 4)))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d, e: f, 1: 2, 3: 4);\
        \n}\
        \n"
        );
    }
    mod empty {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn both() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-merge((), ()))}\
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
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-merge((), (c: d, e: f)))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: d, e: f);\
        \n}\
        \n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-merge((c: d, e: f), ()))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: d, e: f);\
        \n}\
        \n"
            );
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "one_arg", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "map1", error tests are not supported yet.

            // Ignoring "map2", error tests are not supported yet.
            mod nested {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "map1", error tests are not supported yet.

                // Ignoring "map2", error tests are not supported yet.
            }
        }

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: inspect(map-merge($map1: (c: d), $map2: (1: 2)))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d, 1: 2);\
        \n}\
        \n"
        );
    }
    mod nested {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn different_keys() {
            assert_eq!(
        rsass(
            "a {b: inspect(map-merge((c: (d: e, f: g)), c, (1: 2, 3: 4)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: e, f: g, 1: 2, 3: 4));\
        \n}\
        \n"
    );
        }
        mod empty {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn both() {
                assert_eq!(
                    rsass(
                        "a {b: inspect(map-merge((c: ()), c, ()))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: ());\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn first() {
                assert_eq!(
                    rsass(
                        "a {b: inspect(map-merge((c: ()), c, (d: e, f: g)))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: (d: e, f: g));\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn second() {
                assert_eq!(
                    rsass(
                        "a {b: inspect(map-merge((c: (d: e, f: g)), c, ()))}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: (c: (d: e, f: g));\
        \n}\
        \n"
                );
            }
        }
        #[test]
        #[ignore] // unexepected error
        fn intermediate_value_is_not_a_map() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-merge((c: 1), c, d, (e: f)))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: (e: f)));\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn leaf_value_is_not_a_map() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-merge((c: 1), c, (d: e)))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: e));\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn multiple_keys() {
            assert_eq!(
        rsass(
            "a {b: inspect(map-merge((c: (d: (e: (f: (g: h))))), c, d, e, f, (g: 1)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: (e: (f: (g: 1)))));\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn overlapping_keys() {
            assert_eq!(
        rsass(
            "a {b: inspect(map-merge((c: (d: e, f: g, h: i)), c, (j: 1, f: 2, k: 3)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: e, f: 2, h: i, j: 1, k: 3));\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn same_keys() {
            assert_eq!(
        rsass(
            "a {b: inspect(map-merge((c: (d: e, f: g)), c, (d: 1, f: 2)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: 1, f: 2));\
        \n}\
        \n"
    );
        }
    }
    #[test]
    fn overlapping_keys() {
        assert_eq!(
        rsass(
            "a {b: inspect(map-merge((c: d, e: f, g: h), (i: 1, e: 2, j: 3)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: d, e: 2, g: h, i: 1, j: 3);\
        \n}\
        \n"
    );
    }
    #[test]
    fn same_keys() {
        assert_eq!(
            rsass(
                "a {b: inspect(map-merge((c: d, e: f), (c: 1, e: 2)))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: 1, e: 2);\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/map/remove.hrx"
mod remove {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "positional_and_named", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "map", error tests are not supported yet.
        }
    }
    mod found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 1))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (3: 4, 5: 6);\
        \n}\
        \n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 5))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (1: 2, 3: 4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn middle() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 3))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (1: 2, 5: 6);\
        \n}\
        \n"
            );
        }
        mod multiple {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn all() {
                assert_eq!(
        rsass(
            "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6, 7: 8, 9: 10), 1, 5, 9))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (3: 4, 7: 8);\
        \n}\
        \n"
    );
            }
            #[test]
            fn some() {
                assert_eq!(
        rsass(
            "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6, 7: 8), 1, 5, 9))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (3: 4, 7: 8);\
        \n}\
        \n"
    );
            }
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((c: d), c))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: ();\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: inspect(map-remove($map: (c: d), $key: c))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ();\
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
                    "a {b: inspect(map-remove((), 1))}\
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
        fn multiple() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((c: d), e, f, g))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: d);\
        \n}\
        \n"
            );
        }
        #[test]
        fn no_keys() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((c: d)))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: d);\
        \n}\
        \n"
            );
        }
        #[test]
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((c: d), d))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: d);\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/map/set.hrx"
mod set {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn empty() {
        assert_eq!(
            rsass(
                "@use \"sass:map\";\
            \na {b: inspect(map.set((), c, d))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d);\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "one_arg", error tests are not supported yet.

        // Ignoring "two_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "@use \"sass:map\";\
            \na {b: inspect(map.set($map: (c: d), $key: c, $value: e))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: e);\
        \n}\
        \n"
        );
    }
    mod nested {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn empty() {
            assert_eq!(
                rsass(
                    "@use \"sass:map\";\
            \na {b: inspect(map.set((c: ()), c, d, e, f))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: (e: f)));\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn long() {
            assert_eq!(
        rsass(
            "@use \"sass:map\";\
            \na {b: inspect(map.set((c: (d: (e: (f: (g: h))))), c, d, e, f, g, i))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: (e: (f: (g: i)))));\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn new_key() {
            assert_eq!(
                rsass(
                    "@use \"sass:map\";\
            \na {b: inspect(map.set((c: (d: e)), c, f, g))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: e, f: g));\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn update_existing_key() {
            assert_eq!(
                rsass(
                    "@use \"sass:map\";\
            \na {b: inspect(map.set((c: (d: e)), c, d, f))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: f));\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn value_is_not_a_map() {
            assert_eq!(
                rsass(
                    "@use \"sass:map\";\
            \na {b: inspect(map.set((c: 1), c, d, f))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: f));\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn new_key() {
        assert_eq!(
            rsass(
                "@use \"sass:map\";\
            \na {b: inspect(map.set((c: d), e, f))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d, e: f);\
        \n}\
        \n"
        );
    }
    #[test]
    fn update_existing_key() {
        assert_eq!(
            rsass(
                "@use \"sass:map\";\
            \na {b: inspect(map.set((c: d), c, e))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: e);\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/map/values.hrx"
mod values {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn empty() {
        assert_eq!(
            rsass(
                "$result: map-values(());\
            \na {\
            \n  value: inspect($result);\
            \n  separator: list-separator($result);\
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
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn multiple() {
        assert_eq!(
            rsass(
                "a {b: map-values((c: d, e: f, g: h))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: d, f, h;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: map-values($map: (1: 2, 3: 4))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2, 4;\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            rsass(
                "$result: map-values((1: 2));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n  separator: list-separator($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: 2;\
        \n  type: list;\
        \n  separator: comma;\
        \n}\
        \n"
        );
    }
}
