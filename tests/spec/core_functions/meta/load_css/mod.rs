//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/meta/load_css/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;
    mod from_other {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "extend", error tests are not supported yet.

        // Ignoring "runtime", error tests are not supported yet.

        // Ignoring "syntax", error tests are not supported yet.
    }
    mod load {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "test_loop", error tests are not supported yet.

        // Ignoring "missing", error tests are not supported yet.
    }
    mod member {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "global", error tests are not supported yet.

        // Ignoring "namespace", error tests are not supported yet.
    }

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "url", error tests are not supported yet.
        mod with {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "key", error tests are not supported yet.

            // Ignoring "map", error tests are not supported yet.
        }
    }
    mod with {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "conflict", error tests are not supported yet.

        // Ignoring "core_module", error tests are not supported yet.
        mod multi_configuration {
            #[allow(unused)]
            use super::rsass;
            mod double_load {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "both_configured", error tests are not supported yet.

                // Ignoring "through_forward", error tests are not supported yet.

                // Ignoring "unconfigured_first", error tests are not supported yet.
            }
            mod use_and_load {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "both_configured", error tests are not supported yet.

                // Ignoring "load_first", error tests are not supported yet.

                // Ignoring "through_forward", error tests are not supported yet.

                // Ignoring "unconfigured_first", error tests are not supported yet.
            }
        }

        // Ignoring "namespace", error tests are not supported yet.

        // Ignoring "nested", error tests are not supported yet.

        // Ignoring "not_default", error tests are not supported yet.

        // Ignoring "repeated_variable", error tests are not supported yet.
        mod through_forward {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "test_as", error tests are not supported yet.

            // Ignoring "hide", error tests are not supported yet.

            // Ignoring "show", error tests are not supported yet.

            // Ignoring "with", error tests are not supported yet.
        }

        // Ignoring "undefined", error tests are not supported yet.
    }
}

// From "sass-spec/spec/core_functions/meta/load_css/extend.hrx"
mod extend {
    #[allow(unused)]
    use super::rsass;
    mod in_input {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn after() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\");\
            \n\
            \nd {@extend a}\
            \n"
                )
                .unwrap(),
                "a, d {\
        \n  b: c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn before() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n\
            \nd {@extend a}\
            \n@include meta.load-css(\"other\");\
            \n"
                )
                .unwrap(),
                "a, d {\
        \n  b: c;\
        \n}\
        \n"
            );
        }
    }
    mod in_other {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn after() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n\
            \n@include meta.load-css(\"other\");\
            \na {b: c}\
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
        #[ignore] // unexepected error
        fn before() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n\
            \na {b: c}\
            \n@include meta.load-css(\"other\");\
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
}

// From "sass-spec/spec/core_functions/meta/load_css/plain_css.hrx"
mod plain_css {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // unexepected error
    fn at_rule() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\");\
            \n"
            )
            .unwrap(),
            "@media screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
        );
    }
    mod empty {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn built_in() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"sass:color\");\
            \n\
            \n/* No output other than this */\
            \n"
                )
                .unwrap(),
                "/* No output other than this */\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn user_defined() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\");\
            \n\
            \n/* No output other than this */\
            \n"
                )
                .unwrap(),
                "/* No output other than this */\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn named() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css($module: \"other\");\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
    mod nested {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn media_query() {
            assert_eq!(
                rsass(
                    "// Regression test for dart-sass#843\
            \n@use \"sass:meta\";\
            \n@include meta.load-css(\"midstream\")\
            \n"
                )
                .unwrap(),
                "/**/\
        \n@media b {\
        \n  a {\
        \n    c: d;\
        \n  }\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn parent_selector() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \na {@include meta.load-css(\"other\")}\
            \n"
                )
                .unwrap(),
                "a c b {\
        \n  x: y;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn plain_plain_css() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \na {@include meta.load-css(\"other\")}\
            \n"
                )
                .unwrap(),
                "a b {\
        \n  c: d;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn plain_css_import() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n\
            \na {b: c}\
            \n\
            \n@include meta.load-css(\"other\");\
            \n"
            )
            .unwrap(),
            "@import \"style.css\";\
        \na {\
        \n  b: c;\
        \n}\
        \nd {\
        \n  e: f;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn style_rule() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\");\
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
    #[ignore] // unexepected error
    fn through_other_mixin() {
        assert_eq!(
            rsass(
                "@use \"subdir/midstream\";\
            \n@include midstream.load-css(\"upstream\");\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: in subdir;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/meta/load_css/twice.hrx"
mod twice {
    #[allow(unused)]
    use super::rsass;
    mod load_css {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn different_extend() {
            assert_eq!(
                rsass(
                    "@use \"left\";\
            \n@use \"right\";\
            \n"
                )
                .unwrap(),
                "a, left {\
        \n  b: c;\
        \n}\
        \na, right {\
        \n  b: c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn different_nesting() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \na {@include meta.load-css(\"other\")}\
            \nb {@include meta.load-css(\"other\")}\
            \n"
                )
                .unwrap(),
                "a c {\
        \n  d: e;\
        \n}\
        \nb c {\
        \n  d: e;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn runs_once() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\");\
            \n@include meta.load-css(\"other\");\
            \n\
            \n/* No output other than this */\
            \n"
                )
                .unwrap(),
                "/* No output other than this */\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn shares_state() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@use \"shared\";\
            \n@include meta.load-css(\"other\");\
            \n\
            \na {shared-b: shared.$b}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  shared-b: value set by other;\
        \n}\
        \n"
        );
    }
    mod test_use {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn different_extend() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@use \"midstream\";\
            \n@include meta.load-css(\"other\")\
            \n"
                )
                .unwrap(),
                "b, a {\
        \n  c: d;\
        \n}\
        \nb {\
        \n  c: d;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn different_nesting() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@use \"other\";\
            \na {@include meta.load-css(\"other\")}\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: d;\
        \n}\
        \na b {\
        \n  c: d;\
        \n}\
        \n"
            );
        }
        mod runs_once {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn different_text() {
                assert_eq!(
                    rsass(
                        "@use \"sass:meta\";\
            \n@use \"other\";\
            \n@include meta.load-css(\"_other\");\
            \n\
            \n/* No output other than this */\
            \n"
                    )
                    .unwrap(),
                    "/* No output other than this */\
        \n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn same_text() {
                assert_eq!(
                    rsass(
                        "@use \"sass:meta\";\
            \n@use \"other\";\
            \n@include meta.load-css(\"other\");\
            \n\
            \n/* No output other than this */\
            \n"
                    )
                    .unwrap(),
                    "/* No output other than this */\
        \n"
                );
            }
        }
    }
}

// From "sass-spec/spec/core_functions/meta/load_css/with.hrx"
mod with {
    #[allow(unused)]
    use super::rsass;
    mod core_module {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn indirect() {
            assert_eq!(
                rsass(
                    "// Regression test for sass/dart-sass#838.\
            \n@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (c: e));\
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
    #[test]
    #[ignore] // unexepected error
    fn dash_insensitive() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (a_b: configured));\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn doesnt_run_default() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (a: configured));\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn empty() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: ());\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
    mod multi_load {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn empty() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"upstream\", $with: (a: configured));\
            \n\
            \n// An empty configuration map counts as no configuration.\
            \n@include meta.load-css(\"midstream\", $with: ());\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: configured;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn forward() {
            assert_eq!(
        rsass(
            "// This indirection is necessary so that we can execute `meta.load-css()` before\
            \n// we begin loading `used`.\
            \n@use \"loads\";\
            \n@use \"midstream\";\
            \n\
            \nb {c: midstream.$a}\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn test_use() {
            assert_eq!(
        rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"upstream\", $with: (a: configured));\
            \n\
            \n// We have to load this dynamically, because we can\'t have a `@use` after an\
            \n// `@include`.\
            \n@include meta.load-css(\"midstream\");\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn multiple() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (\
            \n  a: configured a,\
            \n  b: configured b,\
            \n  c: configured c\
            \n));\
            \n"
            )
            .unwrap(),
            "d {\
        \n  a: configured a;\
        \n  b: configured b;\
        \n  c: configured c;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn single() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (a: configured));\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn some_unconfigured() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (a: configured a));\
            \n"
            )
            .unwrap(),
            "c {\
        \n  a: configured a;\
        \n  b: original b;\
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
            \n@include meta.load-css(\"loaded\", $with: (b-a: configured));\
            \n"
                )
                .unwrap(),
                "c {\
        \n  d: configured;\
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
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: configured;\
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
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: configured;\
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
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: configured;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn transitive() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: configured;\
        \n}\
        \n"
            );
        }
        mod with {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn default() {
                assert_eq!(
                    rsass(
                        "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: from input));\
            \n"
                    )
                    .unwrap(),
                    "b {\
        \n  c: from input;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn null() {
                assert_eq!(
                    rsass(
                        "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: null));\
            \n"
                    )
                    .unwrap(),
                    "b {\
        \n  c: from loaded;\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn unconfigured() {
                assert_eq!(
                    rsass(
                        "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: from input));\
            \n"
                    )
                    .unwrap(),
                    "c {\
        \n  a: from input;\
        \n  b: from loaded;\
        \n}\
        \n"
                );
            }
        }
    }
    mod through_import {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn direct() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: configured;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn transitive() {
            assert_eq!(
                rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: configured;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn variable_exists() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (a: configured));\
            \n"
            )
            .unwrap(),
            "b {\
        \n  before-declaration: false;\
        \n  after-declaration: true;\
        \n}\
        \n"
        );
    }
}
