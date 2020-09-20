//! Tests auto-converted from "sass-spec/spec/core_functions/meta"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/meta/call.hrx"
mod call {
    #[allow(unused)]
    use super::rsass;
    mod args {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn named() {
            assert_eq!(
        rsass(
            "a {b: call(get-function(\"rgb\"), $blue: 1, $green: 2, $red: 3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #030201;\
        \n}\
        \n"
    );
        }
        #[test]
        fn none() {
            assert_eq!(
                rsass(
                    "@function a() {@return b}\
            \nc {d: call(get-function(\"a\"))}\
            \n"
                )
                .unwrap(),
                "c {\
        \n  d: b;\
        \n}\
        \n"
            );
        }
        #[test]
        fn positional() {
            assert_eq!(
                rsass(
                    "a {b: call(get-function(\"rgb\"), 1, 2, 3)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #010203;\
        \n}\
        \n"
            );
        }
        mod splat {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn combined() {
                assert_eq!(
                    rsass(
                        "$positional: 1 2;\
            \n$named: (\"blue\": 3);\
            \na {b: call(get-function(\"rgb\"), $positional..., $named...)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: #010203;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn named() {
                assert_eq!(
                    rsass(
                        "$args: (\"green\": 1, \"blue\": 2, \"red\": 3);\
            \na {b: call(get-function(\"rgb\"), $args...)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: #030102;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn positional() {
                assert_eq!(
                    rsass(
                        "$args: 1, 2, 3;\
            \na {b: call(get-function(\"rgb\"), $args...)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: #010203;\
        \n}\
        \n"
                );
            }
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "invalid_args", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    #[ignore] // unexepected error
    fn named() {
        assert_eq!(
        rsass(
            "a {b: call($function: get-function(\"rgb\"), $red: 1, $green: 2, $blue: 3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #010203;\
        \n}\
        \n"
    );
    }
    mod string {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn built_in() {
            assert_eq!(
                rsass(
                    "a {b: call(\"rgb\", 1, 2, 3)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #010203;\
        \n}\
        \n"
            );
        }
        #[test]
        fn local() {
            assert_eq!(
                rsass(
                    "@function a($arg) {@return $arg + 1}\
            \na {b: call(\"a\", 1)}\
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
}

// From "sass-spec/spec/core_functions/meta/content_exists.hrx"
mod content_exists {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "in_content", error tests are not supported yet.

        // Ignoring "in_function_called_by_mixin", error tests are not supported yet.

        // Ignoring "outside_mixin", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
    }
    mod test_false {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn through_content() {
            assert_eq!(
                rsass(
                    "@mixin call-content {\
            \n  @content;\
            \n}\
            \n\
            \n@mixin print-content-exists {\
            \n  a {b: content-exists()}\
            \n}\
            \n\
            \n@include call-content {\
            \n  @include print-content-exists;\
            \n}\
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
                    "@mixin a {\
            \n  b {c: content-exists()}\
            \n}\
            \n@include a;\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: false;\
        \n}\
        \n"
            );
        }
    }
    mod test_true {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn empty() {
            assert_eq!(
                rsass(
                    "@mixin a {\
            \n  b {c: content-exists()}\
            \n  @content;\
            \n}\
            \n@include a {}\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn non_empty() {
            assert_eq!(
                rsass(
                    "@mixin a {\
            \n  b {c: content-exists()}\
            \n  @content;\
            \n}\
            \n@include a {\
            \n  d {e: f}\
            \n}\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: true;\
        \n}\
        \nd {\
        \n  e: f;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/meta/feature_exists.hrx"
mod feature_exists {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn at_error() {
        assert_eq!(
            rsass(
                "a {b: feature-exists(at-error)}\
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
    #[ignore] // wrong result
    fn custom_property() {
        assert_eq!(
            rsass(
                "a {b: feature-exists(custom-property)}\
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
    fn dash_sensitive() {
        assert_eq!(
            rsass(
                "a {b: feature-exists(at_error)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
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
    #[ignore] // wrong result
    fn extend_selector_pseudoclass() {
        assert_eq!(
            rsass(
                "a {b: feature-exists(extend-selector-pseudoclass)}\
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
    fn global_variable_shadowing() {
        assert_eq!(
            rsass(
                "a {b: feature-exists(global-variable-shadowing)}\
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
    fn named() {
        assert_eq!(
            rsass(
                "a {b: feature-exists($feature: at-error)}\
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
    fn quote_insensitive() {
        assert_eq!(
            rsass(
                "a {b: feature-exists(\"at-error\")}\
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
    fn units_level_3() {
        assert_eq!(
            rsass(
                "a {b: feature-exists(units-level-3)}\
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
    fn unknown() {
        assert_eq!(
            rsass(
                "a {b: feature-exists(unknown)}\
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

// From "sass-spec/spec/core_functions/meta/function_exists.hrx"
mod function_exists {
    #[allow(unused)]
    use super::rsass;
    mod different_module {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn chosen_prefix() {
            assert_eq!(
                rsass(
                    "@use \"sass:color\" as a;\
            \nb {c: function-exists(\"red\", \"a\")}\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn defined() {
            assert_eq!(
                rsass(
                    "@use \"sass:color\";\
            \na {b: function-exists(\"red\", \"color\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: true;\
        \n}\
        \n"
            );
        }
        mod through_forward {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn test_as() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {\
            \n  with-prefix: function-exists(b-c);\
            \n  without-prefix: function-exists(c);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  with-prefix: true;\
        \n  without-prefix: false;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn bare() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {b: function-exists(c)}\
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
            #[ignore] // wrong result
            fn hide() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {\
            \n  hidden: function-exists(b);\
            \n  not-hidden: function-exists(c);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  hidden: false;\
        \n  not-hidden: true;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn show() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {\
            \n  shown: function-exists(b);\
            \n  not-shown: function-exists(c);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  shown: true;\
        \n  not-shown: false;\
        \n}\
        \n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn through_use() {
            assert_eq!(
                rsass(
                    "@use \"other\" as *;\
            \na {b: function-exists(global-function)}\
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
        #[ignore] // wrong result
        fn undefined() {
            assert_eq!(
                rsass(
                    "@use \"sass:color\";\
            \na {b: function-exists(\"c\", \"color\")}\
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
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod argument {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_few", error tests are not supported yet.

            // Ignoring "too_many", error tests are not supported yet.
            mod test_type {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "module", error tests are not supported yet.

                // Ignoring "name", error tests are not supported yet.
            }
        }

        // Ignoring "conflict", error tests are not supported yet.
        mod module {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "built_in_but_not_loaded", error tests are not supported yet.

            // Ignoring "dash_sensitive", error tests are not supported yet.

            // Ignoring "non_existent", error tests are not supported yet.
        }
    }
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \n\
            \na {b: function-exists($name: \"red\", $module: \"color\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
    mod same_module {
        #[allow(unused)]
        use super::rsass;
        mod dash_insensitive {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn dash_to_underscore() {
                assert_eq!(
                    rsass(
                        "@function a_b() {@return null}\
            \n\
            \nc {d: function-exists(a-b)}\
            \n"
                    )
                    .unwrap(),
                    "c {\
        \n  d: true;\
        \n}\
        \n"
                );
            }
            #[test]
            fn underscore_to_dash() {
                assert_eq!(
                    rsass(
                        "@function a-b() {@return null}\
            \n\
            \nc {d: function-exists(a_b)}\
            \n"
                    )
                    .unwrap(),
                    "c {\
        \n  d: true;\
        \n}\
        \n"
                );
            }
        }
        #[test]
        fn global() {
            assert_eq!(
                rsass(
                    "@function global-function() {@return null}\
            \n\
            \na {b: function-exists(global-function)}\
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
        fn local() {
            assert_eq!(
                rsass(
                    "a {\
            \n  @function local-function() {@return null}\
            \n  b: function-exists(local-function);\
            \n}\
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
        fn non_existent() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: function-exists(non-existent);\
            \n}\
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
        #[ignore] // wrong result
        fn through_import() {
            assert_eq!(
                rsass(
                    "@import \"other\";\
            \na {b: function-exists(global-function)}\
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
}

mod get_function;

// From "sass-spec/spec/core_functions/meta/global_variable_exists.hrx"
mod global_variable_exists {
    #[allow(unused)]
    use super::rsass;
    mod dash_insensitive {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn dash_to_underscore() {
            assert_eq!(
                rsass(
                    "$a_b: null;\
            \n\
            \nc {d: global-variable-exists(a-b)}\
            \n"
                )
                .unwrap(),
                "c {\
        \n  d: true;\
        \n}\
        \n"
            );
        }
        #[test]
        fn underscore_to_dash() {
            assert_eq!(
                rsass(
                    "$a-b: null;\
            \n\
            \nc {d: global-variable-exists(a_b)}\
            \n"
                )
                .unwrap(),
                "c {\
        \n  d: true;\
        \n}\
        \n"
            );
        }
    }
    mod different_module {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn chosen_prefix() {
            assert_eq!(
                rsass(
                    "@use \"other\" as a;\
            \nb {c: global-variable-exists(\"d\", \"a\")}\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn defined() {
            assert_eq!(
                rsass(
                    "@use \"other\";\
            \na {b: global-variable-exists(\"c\", \"other\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: true;\
        \n}\
        \n"
            );
        }
        mod through_forward {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn test_as() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {\
            \n  with-prefix: global-variable-exists(b-c);\
            \n  without-prefix: global-variable-exists(c);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  with-prefix: true;\
        \n  without-prefix: false;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn bare() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {b: variable-exists(c)}\
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
            #[ignore] // wrong result
            fn hide() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {\
            \n  hidden: global-variable-exists(b);\
            \n  not-hidden: global-variable-exists(c);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  hidden: false;\
        \n  not-hidden: true;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn show() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {\
            \n  shown: global-variable-exists(b);\
            \n  not-shown: global-variable-exists(c);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  shown: true;\
        \n  not-shown: false;\
        \n}\
        \n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn through_use() {
            assert_eq!(
                rsass(
                    "@use \"other\" as *;\
            \na {b: global-variable-exists(global-variable)}\
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
        #[ignore] // wrong result
        fn undefined() {
            assert_eq!(
                rsass(
                    "@use \"sass:color\";\
            \na {b: global-variable-exists(\"c\", \"color\")}\
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
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod argument {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_few", error tests are not supported yet.

            // Ignoring "too_many", error tests are not supported yet.
            mod test_type {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "module", error tests are not supported yet.

                // Ignoring "name", error tests are not supported yet.
            }
        }

        // Ignoring "conflict", error tests are not supported yet.
        mod module {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "built_in_but_not_loaded", error tests are not supported yet.

            // Ignoring "dash_sensitive", error tests are not supported yet.

            // Ignoring "non_existent", error tests are not supported yet.
        }
    }
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
            rsass(
                "@use \"other\";\
            \na {b: global-variable-exists($name: \"c\", $module: \"other\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
    mod same_module {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn global() {
            assert_eq!(
                rsass(
                    "$global-variable: null;\
            \n\
            \na {b: global-variable-exists(global-variable)}\
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
        fn local() {
            assert_eq!(
                rsass(
                    "a {\
            \n  $local-variable: null;\
            \n  b: global-variable-exists(local-variable);\
            \n}\
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
        fn non_existent() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: global-variable-exists(non-existent);\
            \n}\
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
        #[ignore] // wrong result
        fn through_import() {
            assert_eq!(
                rsass(
                    "@import \"other\";\
            \na {b: global-variable-exists(global-variable)}\
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
}

// From "sass-spec/spec/core_functions/meta/inspect.hrx"
mod inspect {
    #[allow(unused)]
    use super::rsass;
    mod boolean {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn test_false() {
            assert_eq!(
                rsass(
                    "$result: inspect(false);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: false;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        fn test_true() {
            assert_eq!(
                rsass(
                    "$result: inspect(true);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: true;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
    }
    mod color {
        #[allow(unused)]
        use super::rsass;
        mod generated {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn alpha() {
                assert_eq!(
                    rsass(
                        "$result: inspect(rgba(1, 2, 3, 0.4));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: rgba(1, 2, 3, 0.4);\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn long_hex() {
                assert_eq!(
                    rsass(
                        "@import \"../utils\";\
            \n$result: inspect(generated-color(#abcdef));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: #abcdef;\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn named() {
                assert_eq!(
                    rsass(
                        "@import \"../utils\";\
            \n$result: inspect(generated-color(#00f));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: blue;\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn short_hex() {
                assert_eq!(
                    rsass(
                        "@import \"../utils\";\
            \n$result: inspect(generated-color(#abc));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: #aabbcc;\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn transparent() {
                assert_eq!(
                    rsass(
                        "@import \"../utils\";\
            \n$result: inspect(generated-color(transparent));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: rgba(0, 0, 0, 0);\
        \n  type: string;\
        \n}\
        \n"
                );
            }
        }
        mod literal {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn long_hex() {
                assert_eq!(
                    rsass(
                        "$result: inspect(#0000ff);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: #0000ff;\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            fn named() {
                assert_eq!(
                    rsass(
                        "$result: inspect(blue);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: blue;\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            fn short_hex() {
                assert_eq!(
                    rsass(
                        "$result: inspect(#00f);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: #00f;\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            fn transparent() {
                assert_eq!(
                    rsass(
                        "$result: inspect(transparent);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: transparent;\
        \n  type: string;\
        \n}\
        \n"
                );
            }
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
    }
    #[test]
    fn function() {
        assert_eq!(
            rsass(
                "$result: inspect(get-function(\"get-function\"));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: get-function(\"get-function\");\
        \n  type: string;\
        \n}\
        \n"
        );
    }
    mod list {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn bracketed() {
            assert_eq!(
                rsass(
                    "$result: inspect([1, 2, 3]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: [1, 2, 3];\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        fn comma() {
            assert_eq!(
                rsass(
                    "$result: inspect((1, 2, 3));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: 1, 2, 3;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        #[test]
        fn empty() {
            assert_eq!(
                rsass(
                    "$result: inspect(());\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: ();\
        \n  type: string;\
        \n}\
        \n"
            );
        }
        mod nested {
            #[allow(unused)]
            use super::rsass;
            mod bracketed {
                #[allow(unused)]
                use super::rsass;
                mod in_comma {
                    #[allow(unused)]
                    use super::rsass;
                    #[test]
                    fn bracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect([[1, 2], [3, 4]]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: [[1, 2], [3, 4]];\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                    #[test]
                    #[ignore] // wrong result
                    fn unbracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect(((1, 2), (3, 4)));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: (1, 2), (3, 4);\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                }
                mod in_space {
                    #[allow(unused)]
                    use super::rsass;
                    #[test]
                    fn bracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect([[1, 2] [3, 4]]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: [[1, 2] [3, 4]];\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                    #[test]
                    fn unbracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect([1, 2] [3, 4]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: [1, 2] [3, 4];\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                }
            }
            mod comma {
                #[allow(unused)]
                use super::rsass;
                mod in_comma {
                    #[allow(unused)]
                    use super::rsass;
                    #[test]
                    #[ignore] // wrong result
                    fn bracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect([(1, 2), (3, 4)]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: [(1, 2), (3, 4)];\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                    #[test]
                    #[ignore] // wrong result
                    fn unbracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect(((1, 2), (3, 4)));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: (1, 2), (3, 4);\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                }
                mod in_space {
                    #[allow(unused)]
                    use super::rsass;
                    #[test]
                    fn bracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect([(1, 2) (3, 4)]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: [(1, 2) (3, 4)];\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                    #[test]
                    #[ignore] // wrong result
                    fn unbracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect((1, 2) (3, 4));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: (1, 2) (3, 4);\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                }
            }
            mod empty {
                #[allow(unused)]
                use super::rsass;
                mod in_comma {
                    #[allow(unused)]
                    use super::rsass;
                    #[test]
                    #[ignore] // wrong result
                    fn bracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect([(), ()]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: [(), ()];\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                    #[test]
                    #[ignore] // wrong result
                    fn unbracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect(((), ()));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: (), ();\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                }
                mod in_space {
                    #[allow(unused)]
                    use super::rsass;
                    #[test]
                    #[ignore] // wrong result
                    fn bracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect([() ()]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: [() ()];\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                    #[test]
                    #[ignore] // wrong result
                    fn unbracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect(() ());\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: () ();\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                }
            }
            mod space {
                #[allow(unused)]
                use super::rsass;
                mod in_comma {
                    #[allow(unused)]
                    use super::rsass;
                    #[test]
                    fn bracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect([1 2, 3 4]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: [1 2, 3 4];\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                    #[test]
                    fn unbracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect((1 2, 3 4));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: 1 2, 3 4;\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                }
                mod in_space {
                    #[allow(unused)]
                    use super::rsass;
                    #[test]
                    fn bracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect([(1 2) (3 4)]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: [(1 2) (3 4)];\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                    #[test]
                    #[ignore] // wrong result
                    fn unbracketed() {
                        assert_eq!(
                            rsass(
                                "$result: inspect((1 2) (3 4));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                            )
                            .unwrap(),
                            "a {\
        \n  value: (1 2) (3 4);\
        \n  type: string;\
        \n}\
        \n"
                        );
                    }
                }
            }
        }
        mod single {
            #[allow(unused)]
            use super::rsass;
            mod bracketed {
                #[allow(unused)]
                use super::rsass;
                #[test]
                fn comma() {
                    assert_eq!(
                        rsass(
                            "$result: inspect([1,]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [1,];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn undecided() {
                    assert_eq!(
                        rsass(
                            "$result: inspect([1]);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: [1];\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
            #[test]
            #[ignore] // wrong result
            fn comma() {
                assert_eq!(
                    rsass(
                        "$result: inspect((1,));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: (1,);\
        \n  type: string;\
        \n}\
        \n"
                );
            }
            #[test]
            fn space() {
                assert_eq!(
                    rsass(
                        "$result: inspect(append((), 1, space));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  value: 1;\
        \n  type: string;\
        \n}\
        \n"
                );
            }
        }
        #[test]
        fn space() {
            assert_eq!(
                rsass(
                    "$result: inspect(1 2 3);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: 1 2 3;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
    }
    mod map {
        #[allow(unused)]
        use super::rsass;
        mod list {
            #[allow(unused)]
            use super::rsass;
            mod key {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn comma() {
                    assert_eq!(
                        rsass(
                            "$result: inspect(((1, 2): 3, (4, 5): 6));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: ((1, 2): 3, (4, 5): 6);\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                #[ignore] // wrong result
                fn space() {
                    assert_eq!(
                        rsass(
                            "$result: inspect((1 2: 3, 4 5: 6));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: (1 2: 3, 4 5: 6);\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
            mod value {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn comma() {
                    assert_eq!(
                        rsass(
                            "$result: inspect((1: (2, 3), 4: (5, 6)));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: (1: (2, 3), 4: (5, 6));\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn space() {
                    assert_eq!(
                        rsass(
                            "$result: inspect((1: 2 3, 4: 5 6));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  value: (1: 2 3, 4: 5 6);\
        \n  type: string;\
        \n}\
        \n"
                    );
                }
            }
        }
        #[test]
        fn number() {
            assert_eq!(
                rsass(
                    "$result: inspect((1: 2, 3: 4));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: (1: 2, 3: 4);\
        \n  type: string;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn null() {
        assert_eq!(
            rsass(
                "$result: inspect(null);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: null;\
        \n  type: string;\
        \n}\
        \n"
        );
    }
    mod number {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn unit() {
            assert_eq!(
        rsass(
            "// We explicitly don\'t test the inspect format for complex units. Their format\
            \n// isn\'t guaranteed by the spec, since they can\'t be written literally in Sass.\
            \n$result: inspect(50px);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: 50px;\
        \n  type: string;\
        \n}\
        \n"
    );
        }
        #[test]
        fn unitless() {
            assert_eq!(
                rsass(
                    "$result: inspect(123.456);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: 123.456;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
    }
    mod string {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn quoted() {
            assert_eq!(
        rsass(
            "$result: inspect(\"foo\");\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n\
            \n  // inspect() should always return an unquoted string, so when it\'s passed a\
            \n  // quoted string its return value should contain quote characters. We check\
            \n  // the length to verify that the quotes are included, since there\'s no\
            \n  // built-in way to check whether a string is quoted.\
            \n  length: str-length($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: \"foo\";\
        \n  type: string;\
        \n  length: 5;\
        \n}\
        \n"
    );
        }
        #[test]
        fn unquoted() {
            assert_eq!(
                rsass(
                    "$result: inspect(foo);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  value: foo;\
        \n  type: string;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/meta/keywords.hrx"
mod keywords {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn dash_insensitive() {
        assert_eq!(
            rsass(
                "@import \"../utils\";\
            \na {b: inspect(args-to-keywords($c-d: e, $f_g: h))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c-d: e, f-g: h);\
        \n}\
        \n"
        );
    }
    mod empty {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn no_args() {
            assert_eq!(
                rsass(
                    "@import \"../../utils\";\
            \na {b: inspect(args-to-keywords())}\
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
        fn positional() {
            assert_eq!(
                rsass(
                    "@import \"../../utils\";\
            \na {b: inspect(args-to-keywords(1, 2, 3))}\
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
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "non_arg_list", error tests are not supported yet.

            // Ignoring "non_list", error tests are not supported yet.
        }
    }
    mod forwarded {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn call() {
            assert_eq!(
                rsass(
                    "@import \"../../utils\";\
            \n\
            \n@function args-to-keywords-forward($args...) {\
            \n  @return call(get-function(\"args-to-keywords\"), $args...);\
            \n}\
            \n\
            \na {b: inspect(args-to-keywords-forward($c: d))}\
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
        #[ignore] // unexepected error
        fn content() {
            assert_eq!(
                rsass(
                    "@import \"../../utils\";\
            \n\
            \n@mixin args-to-keywords-forward($args...) {\
            \n  @content($args...);\
            \n}\
            \n\
            \n@include args-to-keywords-forward($c: d) using ($args...) {\
            \n  a {b: inspect(args-to-keywords($args...))}\
            \n}\
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
        #[ignore] // wrong result
        fn function() {
            assert_eq!(
                rsass(
                    "@import \"../../utils\";\
            \n\
            \n@function args-to-keywords-forward($args...) {\
            \n  @return args-to-keywords($args...);\
            \n}\
            \n\
            \na {b: inspect(args-to-keywords-forward($c: d))}\
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
        #[ignore] // wrong result
        fn mixin() {
            assert_eq!(
                rsass(
                    "@import \"../../utils\";\
            \n\
            \n@mixin args-to-keywords-forward($args...) {\
            \n  a {b: inspect(args-to-keywords($args...))}\
            \n}\
            \n\
            \n@include args-to-keywords-forward($c: d);\
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
    #[test]
    #[ignore] // wrong result
    fn multi_arg() {
        assert_eq!(
            rsass(
                "@import \"../utils\";\
            \na {b: inspect(args-to-keywords($c: d, $e: f, $g: h))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d, e: f, g: h);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
            rsass(
                "@function args-to-keywords($args...) {\
            \n  @return keywords($args: $args);\
            \n}\
            \n\
            \na {b: inspect(args-to-keywords($c: d))}\
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
    #[ignore] // wrong result
    fn one_arg() {
        assert_eq!(
            rsass(
                "@import \"../utils\";\
            \na {b: inspect(args-to-keywords($c: d))}\
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

mod load_css;

// From "sass-spec/spec/core_functions/meta/mixin_exists.hrx"
mod mixin_exists {
    #[allow(unused)]
    use super::rsass;
    mod different_module {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn chosen_prefix() {
            assert_eq!(
                rsass(
                    "@use \"other\" as a;\
            \nb {c: mixin-exists(\"d\", \"a\")}\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn defined() {
            assert_eq!(
                rsass(
                    "@use \"other\";\
            \na {b: mixin-exists(\"c\", \"other\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: true;\
        \n}\
        \n"
            );
        }
        mod through_forward {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn test_as() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {\
            \n  with-prefix: mixin-exists(b-c);\
            \n  without-prefix: mixin-exists(c);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  with-prefix: true;\
        \n  without-prefix: false;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn bare() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {b: mixin-exists(c)}\
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
            #[ignore] // wrong result
            fn hide() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {\
            \n  hidden: mixin-exists(b);\
            \n  not-hidden: mixin-exists(c);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  hidden: false;\
        \n  not-hidden: true;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn show() {
                assert_eq!(
                    rsass(
                        "@use \"midstream\" as *;\
            \na {\
            \n  shown: mixin-exists(b);\
            \n  not-shown: mixin-exists(c);\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  shown: true;\
        \n  not-shown: false;\
        \n}\
        \n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn through_use() {
            assert_eq!(
                rsass(
                    "@use \"other\" as *;\
            \na {b: mixin-exists(global-mixin)}\
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
        #[ignore] // wrong result
        fn undefined() {
            assert_eq!(
                rsass(
                    "@use \"sass:color\";\
            \na {b: mixin-exists(\"c\", \"color\")}\
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
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod argument {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_few", error tests are not supported yet.

            // Ignoring "too_many", error tests are not supported yet.
            mod test_type {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "module", error tests are not supported yet.

                // Ignoring "name", error tests are not supported yet.
            }
        }

        // Ignoring "conflict", error tests are not supported yet.
        mod module {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "built_in_but_not_loaded", error tests are not supported yet.

            // Ignoring "dash_sensitive", error tests are not supported yet.

            // Ignoring "non_existent", error tests are not supported yet.
        }
    }
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
            rsass(
                "@use \"other\";\
            \na {b: mixin-exists($name: \"c\", $module: \"other\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
    mod same_module {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn global() {
            assert_eq!(
                rsass(
                    "@mixin global-mixin() {}\
            \n\
            \na {b: mixin-exists(global-mixin)}\
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
        fn local() {
            assert_eq!(
                rsass(
                    "a {\
            \n  @mixin local-mixin() {}\
            \n  b: mixin-exists(local-mixin);\
            \n}\
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
        fn non_existent() {
            assert_eq!(
                rsass(
                    "a {\
            \n  b: mixin-exists(non-existent);\
            \n}\
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
        #[ignore] // wrong result
        fn through_import() {
            assert_eq!(
                rsass(
                    "@import \"other\";\
            \na {b: mixin-exists(global-mixin)}\
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
}

// From "sass-spec/spec/core_functions/meta/module_functions.hrx"
mod module_functions {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // unexepected error
    fn test_as() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@use \"../util\";\
            \n@use \"other\" as b;\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"b\"))\
            \n"
            )
            .unwrap(),
            "a {\
        \n  c: c value;\
        \n  d: d value;\
        \n  e: e value;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn core_module() {
        assert_eq!(
        rsass(
            "@use \"sass:map\";\
            \n@use \"sass:meta\";\
            \n\
            \n// We don\'t want to print every function name in this module, since that would\
            \n// make this test brittle when new functions are added. Instead we just test\
            \n// that a couple functions work.\
            \n\
            \n$functions: meta.module-functions(\"meta\");\
            \na {\
            \n  variable-exists: meta.call(map.get($functions, \"variable-exists\"), \"functions\");\
            \n  inspect: meta.call(map.get($functions, \"inspect\"), ());\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  variable-exists: true;\
        \n  inspect: ();\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn dash_sensitive() {
        assert_eq!(
        rsass(
            "@use \"sass:meta\";\
            \n@use \"../util\";\
            \n@use \"other\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"other\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b-c: b-c value;\
        \n  d-e: d_e value;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn empty() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-functions(\"other\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ();\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "before_load", error tests are not supported yet.

        // Ignoring "dash_sensitive", error tests are not supported yet.

        // Ignoring "global", error tests are not supported yet.

        // Ignoring "missing", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    #[ignore] // unexepected error
    fn multiple() {
        assert_eq!(
        rsass(
            "@use \"sass:meta\";\
            \n@use \"../util\";\
            \n@use \"other\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"other\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b value;\
        \n  c: c value;\
        \n  d: d value;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn named() {
        assert_eq!(
        rsass(
            "@use \"sass:meta\";\
            \n@use \"../util\";\
            \n@use \"other\";\
            \n\
            \n@include util.print-function-map(meta.module-functions($module: \"other\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b value;\
        \n  c: c value;\
        \n  d: d value;\
        \n}\
        \n"
    );
    }
    mod through_forward {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn test_as() {
            assert_eq!(
        rsass(
            "@use \"sass:meta\";\
            \n@use \"../../util\";\
            \n@use \"used\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"used\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b-c: c value;\
        \n  b-d: d value;\
        \n  b-e: e value;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn bare() {
            assert_eq!(
        rsass(
            "@use \"sass:meta\";\
            \n@use \"../../util\";\
            \n@use \"used\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"used\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b value;\
        \n  c: c value;\
        \n  d: d value;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn hide() {
            assert_eq!(
        rsass(
            "@use \"sass:meta\";\
            \n@use \"../../util\";\
            \n@use \"used\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"used\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  d: d value;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn show() {
            assert_eq!(
        rsass(
            "@use \"sass:meta\";\
            \n@use \"../../util\";\
            \n@use \"used\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"used\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b value;\
        \n  c: c value;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn through_import() {
        assert_eq!(
        rsass(
            "@use \"sass:meta\";\
            \n@use \"../util\";\
            \n@use \"used\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"used\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b value;\
        \n  c: c value;\
        \n  d: d value;\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/core_functions/meta/module_variables.hrx"
mod module_variables {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn test_as() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@use \"other\" as a;\
            \n\
            \nb {c: meta.inspect(meta.module-variables(\"a\"))}\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: (\"d\": d value, \"e\": e value, \"f\": f value);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn core_module() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"meta\"))}\
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
    fn dash_sensitive() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"other\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"c-d\": c-d value, \"e-f\": e_f value);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn empty() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"other\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ();\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "before_load", error tests are not supported yet.

        // Ignoring "dash_sensitive", error tests are not supported yet.

        // Ignoring "global", error tests are not supported yet.

        // Ignoring "missing", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn multiple() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"other\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables($module: \"other\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
        \n}\
        \n"
        );
    }
    mod through_forward {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn test_as() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (\"c-d\": d value, \"c-e\": e value, \"c-f\": f value);\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn bare() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn hide() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (\"e\": e value);\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn show() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (\"c\": c value, \"d\": d value);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn through_import() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/meta/type_of.hrx"
mod type_of {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn arglist() {
        assert_eq!(
            rsass(
                "@function type-of-arglist($args...) {\
            \n  @return type-of($args);\
            \n}\
            \n\
            \na {b: type-of-arglist()}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: arglist;\
        \n}\
        \n"
        );
    }
    mod boolean {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn test_false() {
            assert_eq!(
                rsass(
                    "a {b: type-of(false)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: bool;\
        \n}\
        \n"
            );
        }
        #[test]
        fn test_true() {
            assert_eq!(
                rsass(
                    "a {b: type-of(true)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: bool;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn color() {
        assert_eq!(
            rsass(
                "a {b: type-of(red)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: color;\
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
    fn function() {
        assert_eq!(
            rsass(
                "a {b: type-of(get-function(\"type-of\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: function;\
        \n}\
        \n"
        );
    }
    mod list {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: type-of(())}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: list;\
        \n}\
        \n"
            );
        }
        #[test]
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: type-of(1 2 3)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: list;\
        \n}\
        \n"
            );
        }
    }
    mod map {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: type-of(map-remove((c: d), c))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: map;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: type-of((c: d))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: map;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: type-of($value: c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: string;\
        \n}\
        \n"
        );
    }
    #[test]
    fn null() {
        assert_eq!(
            rsass(
                "a {b: type-of(null)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: null;\
        \n}\
        \n"
        );
    }
    mod number {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn unit() {
            assert_eq!(
                rsass(
                    "a {b: type-of(1.5px * 3.4em)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: number;\
        \n}\
        \n"
            );
        }
        #[test]
        fn unitless() {
            assert_eq!(
                rsass(
                    "a {b: type-of(1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: number;\
        \n}\
        \n"
            );
        }
    }
    mod string {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn quoted() {
            assert_eq!(
                rsass(
                    "a {b: type-of(\"c\")}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: string;\
        \n}\
        \n"
            );
        }
        #[test]
        fn unquoted() {
            assert_eq!(
                rsass(
                    "a {b: type-of(c)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: string;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/meta/variable_exists.hrx"
mod variable_exists {
    #[allow(unused)]
    use super::rsass;

    // Ignoring "conflict", error tests are not supported yet.
    mod dash_insensitive {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn dash_to_underscore() {
            assert_eq!(
                rsass(
                    "$a_b: null;\
            \n\
            \nc {d: variable-exists(a-b)}\
            \n"
                )
                .unwrap(),
                "c {\
        \n  d: true;\
        \n}\
        \n"
            );
        }
        #[test]
        fn underscore_to_dash() {
            assert_eq!(
                rsass(
                    "$a-b: null;\
            \n\
            \nc {d: variable-exists(a_b)}\
            \n"
                )
                .unwrap(),
                "c {\
        \n  d: true;\
        \n}\
        \n"
            );
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod argument {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_few", error tests are not supported yet.

            // Ignoring "too_many", error tests are not supported yet.

            // Ignoring "test_type", error tests are not supported yet.
        }
    }
    #[test]
    fn global() {
        assert_eq!(
            rsass(
                "$global-variable: null;\
            \n\
            \na {b: variable-exists(global-variable)}\
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
    fn keyword() {
        assert_eq!(
            rsass(
                "a {b: variable-exists($name: foo)}\
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
    fn local() {
        assert_eq!(
            rsass(
                "a {\
            \n  $local-variable: null;\
            \n  b: variable-exists(local-variable);\
            \n}\
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
    fn non_existent() {
        assert_eq!(
            rsass(
                "a {\
            \n  b: variable-exists(non-existent);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
    mod through_forward {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn test_as() {
            assert_eq!(
                rsass(
                    "@use \"midstream\" as *;\
            \na {\
            \n  with-prefix: variable-exists(b-c);\
            \n  without-prefix: variable-exists(c);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  with-prefix: true;\
        \n  without-prefix: false;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn hide() {
            assert_eq!(
                rsass(
                    "@use \"midstream\" as *;\
            \na {\
            \n  hidden: variable-exists(b);\
            \n  not-hidden: variable-exists(c);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  hidden: false;\
        \n  not-hidden: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn show() {
            assert_eq!(
                rsass(
                    "@use \"midstream\" as *;\
            \na {\
            \n  shown: variable-exists(b);\
            \n  not-shown: variable-exists(c);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  shown: true;\
        \n  not-shown: false;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn through_import() {
        assert_eq!(
            rsass(
                "@import \"other\";\
            \na {b: variable-exists(global-variable)}\
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
    #[ignore] // wrong result
    fn through_use() {
        assert_eq!(
            rsass(
                "@use \"other\" as *;\
            \na {b: variable-exists(global-variable)}\
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
