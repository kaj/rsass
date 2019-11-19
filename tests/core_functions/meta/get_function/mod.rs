//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/meta/get_function/different_module.hrx"
mod different_module {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn chosen_prefix() {
        assert_eq!(
        rsass(
            "@use \"sass:color\" as a;\
            \nb {c: call(get-function(\"red\", $module: \"a\"), #abcdef)}\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: 171;\
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
            \na {b: call(get-function(\"red\", $module: \"color\"), #abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 171;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
        rsass(
            "@use \"sass:color\";\
            \na {b: call(get-function($name: \"red\", $module: \"color\"), #abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 171;\
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
                    "@use \"midstream\" as *;\
                     \na {\
                     \n  b: call(get-function(c-d));\
                     \n}\
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
        #[ignore] // unexepected error
        fn bare() {
            assert_eq!(
                rsass(
                    "@use \"midstream\" as *;\
                     \na {b: call(get-function(c))}\
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
        fn hide() {
            assert_eq!(
                rsass(
                    "@use \"midstream\" as *;\
                     \na {\
                     \n  b: call(get-function(d));\
                     \n}\
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
        #[ignore] // unexepected error
        fn show() {
            assert_eq!(
                rsass(
                    "@use \"midstream\" as *;\
                     \na {\
                     \n  b: call(get-function(c));\
                     \n}\
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
    #[test]
    #[ignore] // unexepected error
    fn through_use() {
        assert_eq!(
            rsass(
                "@use \"other\" as *;\
                 \na {b: call(get-function(add-two), 10)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 12;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/meta/get_function/equality.hrx"
mod equality {
    #[allow(unused)]
    use super::rsass;
    mod built_in {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn different() {
            assert_eq!(
                rsass(
                    "a {b: get-function(lighten) == get-function(darken)}\
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
        fn same() {
            assert_eq!(
                rsass(
                    "a {b: get-function(lighten) == get-function(lighten)}\
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
    fn same_value() {
        assert_eq!(
            rsass(
                "$lighten-fn: get-function(lighten);\
                 \na {b: $lighten-fn == $lighten-fn}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    mod user_defined {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn different() {
            assert_eq!(
        rsass(
            "@function user-defined-1() {@return null}\
            \n@function user-defined-2() {@return null}\
            \na {b: get-function(user-defined-1) == get-function(user-defined-2)}\
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
        fn redefined() {
            assert_eq!(
                rsass(
                    "@function user-defined() {@return null}\
                     \n$first-reference: get-function(user-defined);\
                     \n\
                     \n@function user-defined() {@return null}\
                     \n$second-reference: get-function(user-defined);\
                     \na {b: $first-reference == $second-reference}\
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
        fn same() {
            assert_eq!(
        rsass(
            "@function user-defined() {@return null}\
            \na {b: get-function(user-defined) == get-function(user-defined)}\
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

// From "sass-spec/spec/core_functions/meta/get_function/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;
    mod argument {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "function_ref", error tests are not supported yet.

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

    // Ignoring "division", error tests are not supported yet.

    // Ignoring "function_exists", error tests are not supported yet.
    mod module {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "and_css", error tests are not supported yet.

        // Ignoring "built_in_but_not_loaded", error tests are not supported yet.

        // Ignoring "dash_sensitive", error tests are not supported yet.

        // Ignoring "non_existent", error tests are not supported yet.

        // Ignoring "undefined", error tests are not supported yet.
    }

    // Ignoring "non_existent", error tests are not supported yet.
    mod through_forward {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "hide", error tests are not supported yet.

        // Ignoring "show", error tests are not supported yet.
    }
}

// From "sass-spec/spec/core_functions/meta/get_function/meta.hrx"
mod meta {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn inspect() {
        assert_eq!(
            rsass(
                "a {b: inspect(get-function(lighten))};\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: get-function(\"lighten\");\
             \n}\
             \n"
        );
    }
    #[test]
    fn type_of() {
        assert_eq!(
            rsass(
                "a {b: type-of(get-function(lighten))};\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: function;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/meta/get_function/same_module.hrx"
mod same_module {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn built_in() {
        assert_eq!(
            rsass(
                "$lighten-fn: get-function(lighten);\
                 \n\
                 \na {b: call($lighten-fn, red, 30%)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: #ff9999;\
             \n}\
             \n"
        );
    }
    mod dash_insensitive {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn dash_to_underscore() {
            assert_eq!(
                rsass(
                    "@function add_two($v) {@return $v + 2}\
                     \n\
                     \na {b: call(get-function(add-two), 10)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 12;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn underscore_to_dash() {
            assert_eq!(
                rsass(
                    "@function add-two($v) {@return $v + 2}\
                     \n\
                     \na {b: call(get-function(add_two), 10)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 12;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    fn plain_css() {
        assert_eq!(
            rsass(
                "$sass-fn: get-function(lighten);\
                 \n$css-fn: get-function(lighten, $css: true);\
                 \n\
                 \na {\
                 \n  sass-fn: call($sass-fn, red, 30%);\
                 \n  css-fn: call($css-fn, red, 30%);\
                 \n}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  sass-fn: #ff9999;\
             \n  css-fn: lighten(red, 30%);\
             \n}\
             \n"
        );
    }
    #[test]
    fn redefined() {
        assert_eq!(
        rsass(
            "@function add-two($v) {@return $v + 2}\
            \n$add-two-fn: get-function(add-two);\
            \n\
            \n// The function returned by `get-function()` is locked in place when it\'s\
            \n// called. Redefining the function after the fact shouldn\'t affect the stored\
            \n// value.\
            \n@function add-two($v) {@error \"Should not be called\"}\
            \n\
            \na {b: call($add-two-fn, 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 12;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn through_import() {
        assert_eq!(
            rsass(
                "@import \"other\";\
                 \na {b: call(get-function(add-two), 10)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 12;\
             \n}\
             \n"
        );
    }
    #[test]
    fn user_defined() {
        assert_eq!(
            rsass(
                "@function add-two($v) {@return $v + 2}\
                 \n$add-two-fn: get-function(add-two);\
                 \n\
                 \na {b: call($add-two-fn, 10)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 12;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/meta/get_function/scope.hrx"
mod scope {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn captures_inner_scope() {
        assert_eq!(
        rsass(
            "@function add-two($v) {@error \"Should not be called\"}\
            \n.scope1 {\
            \n  @function add-two($v) {@error \"Should not be called\"}\
            \n  .scope2 {\
            \n    @function add-two($v) {@error \"Should not be called\"}\
            \n    .scope3 {\
            \n      @function add-two($v) {@return $v + 2}\
            \n\
            \n      // Like a normal function call, get-function() will always use the\
            \n      // innermost definition of a function.\
            \n      a: call(get-function(add-two), 10);\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".scope1 .scope2 .scope3 {\
        \n  a: 12;\
        \n}\
        \n"
    );
    }
    #[test]
    fn stores_local_scope() {
        assert_eq!(
        rsass(
            "$add-two-fn: null;\
            \n\
            \n.scope {\
            \n  @function add-two($v) {@return $v + 2}\
            \n\
            \n  // This function reference will still refer to this nested `add-two` function\
            \n  // even when it goes out of scope.\
            \n  $add-two-fn: get-function(add-two) !global;\
            \n}\
            \n\
            \na {b: call($add-two-fn, 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 12;\
        \n}\
        \n"
    );
    }
}
