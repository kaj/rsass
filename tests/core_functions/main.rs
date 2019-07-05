//! Tests auto-converted from "sass-spec/spec/core_functions"
//! version d8dc38cd, 2019-07-03 17:55:12 -0700.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputStyle};

mod color;

mod content_exists;

// From "sass-spec/spec/core_functions/feature_exists.hrx"
#[test]
#[ignore] // failing
fn feature_exists() {
    assert_eq!(
        rsass(
            ".feature-exists {\n  custom-property: feature-exists(custom-property);\n}"
        )
        .unwrap(),
        ".feature-exists {\n  custom-property: true;\n}\n"
    );
}

// From "sass-spec/spec/core_functions/function_exists.hrx"
mod function_exists {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn dash_insensitive() {
        assert_eq!(
        rsass(
            "@function global-function() {@return null}\n\na {b: function-exists(global_function)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
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
            "@function global-function() {@return null}\n\na {b: function-exists(global-function)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
    }
    #[test]
    fn keyword() {
        assert_eq!(
            rsass("a {b: function-exists($name: foo)}\n").unwrap(),
            "a {\n  b: false;\n}\n"
        );
    }
    #[test]
    fn local() {
        assert_eq!(
        rsass(
            "a {\n  @function local-function() {@return null}\n  b: function-exists(local-function);\n}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
    }
    #[test]
    fn non_existent() {
        assert_eq!(
            rsass("a {\n  b: function-exists(non-existent);\n}\n").unwrap(),
            "a {\n  b: false;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn through_import() {
        assert_eq!(
        rsass(
            "@import \"other\";\na {b: function-exists(global-function)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
    }
}

// From "sass-spec/spec/core_functions/get_function.hrx"
mod get_function {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn built_in() {
        assert_eq!(
        rsass(
            "$lighten-fn: get-function(lighten);\n\na {b: call($lighten-fn, red, 30%)}\n"
        )
        .unwrap(),
        "a {\n  b: #ff9999;\n}\n"
    );
    }
    mod equality {
        #[allow(unused)]
        use super::rsass;
        mod built_in {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn different() {
                assert_eq!(
        rsass("a {b: get-function(lighten) == get-function(darken)}\n")
            .unwrap(),
        "a {\n  b: false;\n}\n"
    );
            }
            #[test]
            #[ignore] // failing
            fn same() {
                assert_eq!(
        rsass("a {b: get-function(lighten) == get-function(lighten)}\n")
            .unwrap(),
        "a {\n  b: true;\n}\n"
    );
            }
        }
        #[test]
        #[ignore] // failing
        fn same_value() {
            assert_eq!(
        rsass(
            "$lighten-fn: get-function(lighten);\na {b: $lighten-fn == $lighten-fn}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
        }
        mod user_defined {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn different() {
                assert_eq!(
        rsass(
            "@function user-defined-1() {@return null}\n@function user-defined-2() {@return null}\na {b: get-function(user-defined-1) == get-function(user-defined-2)}\n"
        )
        .unwrap(),
        "a {\n  b: false;\n}\n"
    );
            }
            #[test]
            fn redefined() {
                assert_eq!(
        rsass(
            "@function user-defined() {@return null}\n$first-reference: get-function(user-defined);\n\n@function user-defined() {@return null}\n$second-reference: second-function(user-defined);\na {b: $first-reference == $second-reference}\n"
        )
        .unwrap(),
        "a {\n  b: false;\n}\n"
    );
            }
            #[test]
            fn same() {
                assert_eq!(
        rsass(
            "@function user-defined() {@return null}\na {b: get-function(user-defined) == get-function(user-defined)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
            }
        }
    }
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

                // Ignoring "name", error tests are not supported yet.
            }
        }

        // Ignoring "division", error tests are not supported yet.

        // Ignoring "function_exists", error tests are not supported yet.

        // Ignoring "non_existent", error tests are not supported yet.
    }
    #[test]
    #[ignore] // failing
    fn inspect() {
        assert_eq!(
            rsass("a {b: inspect(get-function(lighten))};\n").unwrap(),
            "a {\n  b: get-function(\"lighten\");\n}\n"
        );
    }
    #[test]
    fn plain_css() {
        assert_eq!(
        rsass(
            "$sass-fn: get-function(lighten);\n$css-fn: get-function(lighten, $css: true);\n\na {\n  sass-fn: call($sass-fn, red, 30%);\n  css-fn: call($css-fn, red, 30%);\n}\n"
        )
        .unwrap(),
        "a {\n  sass-fn: #ff9999;\n  css-fn: lighten(red, 30%);\n}\n"
    );
    }
    #[test]
    #[ignore] // failing
    fn redefined() {
        assert_eq!(
        rsass(
            "@function add-two($v) {@return $v + 2}\n$add-two-fn: get-function(add-two);\n\n// The function returned by `get-function()` is locked in place when it\'s\n// called. Redefining the function after the fact shouldn\'t affect the stored\n// value.\n@function add-two($v) {@error \"Should not be called\"}\n\na {b: call($add-two-fn, 10)}\n"
        )
        .unwrap(),
        "a {\n  b: 12;\n}\n"
    );
    }
    mod scope {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn captures_inner_scope() {
            assert_eq!(
        rsass(
            "@function add-two($v) {@error \"Should not be called\"}\n.scope1 {\n  @function add-two($v) {@error \"Should not be called\"}\n  .scope2 {\n    @function add-two($v) {@error \"Should not be called\"}\n    .scope3 {\n      @function add-two($v) {@return $v + 2}\n\n      // Like a normal function call, get-function() will always use the\n      // innermost definition of a function.\n      a: call(get-function(add-two), 10);\n    }\n  }\n}\n"
        )
        .unwrap(),
        ".scope1 .scope2 .scope3 {\n  a: 12;\n}\n"
    );
        }
        #[test]
        fn stores_local_scope() {
            assert_eq!(
        rsass(
            "$add-two-fn: null;\n\n.scope {\n  @function add-two($v) {@return $v + 2}\n\n  // This function reference will still refer to this nested `add-two` function\n  // even when it goes out of scope.\n  $add-two-fn: get-function(add-two) !global;\n}\n\na {b: call($add-two-fn, 10)}\n"
        )
        .unwrap(),
        "a {\n  b: 12;\n}\n"
    );
        }
    }
    #[test]
    #[ignore] // failing
    fn through_import() {
        assert_eq!(
        rsass("@import \"other\";\na {b: call(get-function(add-two), 10)}\n")
            .unwrap(),
        "a {\n  b: 12;\n}\n"
    );
    }
    #[test]
    #[ignore] // failing
    fn type_of() {
        assert_eq!(
            rsass("a {b: type-of(get-function(lighten))};\n").unwrap(),
            "a {\n  b: function;\n}\n"
        );
    }
    #[test]
    fn user_defined() {
        assert_eq!(
        rsass(
            "@function add-two($v) {@return $v + 2}\n$add-two-fn: get-function(add-two);\n\na {b: call($add-two-fn, 10)}\n"
        )
        .unwrap(),
        "a {\n  b: 12;\n}\n"
    );
    }
}

// From "sass-spec/spec/core_functions/global_variable_exists.hrx"
mod global_variable_exists {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn dash_insensitive() {
        assert_eq!(
        rsass(
            "$global-variable: null;\n\na {b: global-variable-exists(global_variable)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
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
            "$global-variable: null;\n\na {b: global-variable-exists(global-variable)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
    }
    #[test]
    fn keyword() {
        assert_eq!(
            rsass("a {b: global-variable-exists($name: foo)}\n").unwrap(),
            "a {\n  b: false;\n}\n"
        );
    }
    #[test]
    fn local() {
        assert_eq!(
        rsass(
            "a {\n  $local-variable: null;\n  b: global-variable-exists(local-variable);\n}\n"
        )
        .unwrap(),
        "a {\n  b: false;\n}\n"
    );
    }
    #[test]
    fn non_existent() {
        assert_eq!(
            rsass("a {\n  b: global-variable-exists(non-existent);\n}\n")
                .unwrap(),
            "a {\n  b: false;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn through_import() {
        assert_eq!(
        rsass(
            "@import \"other\";\na {b: global-variable-exists(global-variable)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
    }
}

// From "sass-spec/spec/core_functions/inspect.hrx"
mod inspect {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn boolean() {
        assert_eq!(
        rsass(
            "a {\n  $result: inspect(true);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: true;\n  type: string;\n}\n"
    );
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
            "a {\n  $result: inspect(rgba(1, 2, 3, 0.4));\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: rgba(1, 2, 3, 0.4);\n  type: string;\n}\n"
    );
            }
            #[test]
            fn long_hex() {
                assert_eq!(
        rsass(
            "a {\n  $result: inspect(rgb(171, 205, 239));\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: #abcdef;\n  type: string;\n}\n"
    );
            }
            #[test]
            fn named() {
                assert_eq!(
        rsass(
            "a {\n  // This scale-color() call is a no-op, but it will return a non-literal color\n  // value.\n  $result: inspect(scale-color(#00f, $blue: 0%));\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: blue;\n  type: string;\n}\n"
    );
            }
            #[test]
            fn short_hex() {
                assert_eq!(
        rsass(
            "a {\n  // This scale-color() call is a no-op, but it will return a non-literal color\n  // value.\n  $result: inspect(scale-color(#abc, $blue: 0%));\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: #aabbcc;\n  type: string;\n}\n"
    );
            }
            #[test]
            fn transparent() {
                assert_eq!(
        rsass(
            "a {\n  // This scale-color() call is a no-op, but it will return a non-literal color\n  // value.\n  $result: inspect(scale-color(transparent, $blue: 0%));\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: rgba(0, 0, 0, 0);\n  type: string;\n}\n"
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
            "a {\n  $result: inspect(#0000ff);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: #0000ff;\n  type: string;\n}\n"
    );
            }
            #[test]
            fn named() {
                assert_eq!(
        rsass(
            "a {\n  $result: inspect(blue);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: blue;\n  type: string;\n}\n"
    );
            }
            #[test]
            fn short_hex() {
                assert_eq!(
        rsass(
            "a {\n  $result: inspect(#00f);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: #00f;\n  type: string;\n}\n"
    );
            }
            #[test]
            fn transparent() {
                assert_eq!(
        rsass(
            "a {\n  $result: inspect(transparent);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: transparent;\n  type: string;\n}\n"
    );
            }
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "no_args", error tests are not supported yet.

        // Ignoring "two_args", error tests are not supported yet.
    }
    mod list {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn bracketed() {
            assert_eq!(
        rsass(
            "a {\n  $result: inspect([1, 2, 3]);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: [1, 2, 3];\n  type: string;\n}\n"
    );
        }
        #[test]
        fn comma() {
            assert_eq!(
        rsass(
            "a {\n  $result: inspect((1, 2, 3));\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: 1, 2, 3;\n  type: string;\n}\n"
    );
        }
        #[test]
        #[ignore] // failing
        fn empty() {
            assert_eq!(
        rsass(
            "a {\n  $result: inspect(());\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: ();\n  type: string;\n}\n"
    );
        }
        #[test]
        #[ignore] // failing
        fn nested() {
            assert_eq!(
        rsass(
            "a {\n  $a: inspect([() ()]);\n  a: $a;\n  b: type-of($a);\n}\n"
        )
        .unwrap(),
        "a {\n  a: [() ()];\n  b: string;\n}\n"
    );
        }
        #[test]
        #[ignore] // failing
        fn single_with_comma() {
            assert_eq!(
        rsass(
            "a {\n  $result: inspect((1,));\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: (1,);\n  type: string;\n}\n"
    );
        }
        #[test]
        fn space() {
            assert_eq!(
        rsass(
            "a {\n  $result: inspect(1 2 3);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: 1 2 3;\n  type: string;\n}\n"
    );
        }
    }
    #[test]
    fn map() {
        assert_eq!(
        rsass(
            "a {\n  $result: inspect((1: 2, 3: 4));\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: (1: 2, 3: 4);\n  type: string;\n}\n"
    );
    }
    #[test]
    #[ignore] // failing
    fn null() {
        assert_eq!(
        rsass(
            "a {\n  $result: inspect(null);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: null;\n  type: string;\n}\n"
    );
    }
    mod number {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn unit() {
            assert_eq!(
        rsass(
            "// We explicitly don\'t test the inspect format for complex units. Their format\n// isn\'t guaranteed by the spec, since they can\'t be written literally in Sass.\na {\n  $result: inspect(50px);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: 50px;\n  type: string;\n}\n"
    );
        }
        #[test]
        fn unitless() {
            assert_eq!(
        rsass(
            "a {\n  $result: inspect(123.456);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: 123.456;\n  type: string;\n}\n"
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
            "a {\n  $result: inspect(\"foo\");\n  result: $result;\n  type: type-of($result);\n\n  // inspect() should always return an unquoted string, so when it\'s passed a\n  // quoted string its return value should contain quote characters. We check\n  // the length to verify that the quotes are included, since there\'s no\n  // built-in way to check whether a string is quoted.\n  length: str-length($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: \"foo\";\n  type: string;\n  length: 5;\n}\n"
    );
        }
        #[test]
        fn unquoted() {
            assert_eq!(
        rsass(
            "a {\n  $result: inspect(foo);\n  result: $result;\n  type: type-of($result);\n}\n"
        )
        .unwrap(),
        "a {\n  result: foo;\n  type: string;\n}\n"
    );
        }
    }
}

mod list;

mod map;

mod math;

// From "sass-spec/spec/core_functions/mixin_exists.hrx"
mod mixin_exists {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn dash_insensitive() {
        assert_eq!(
        rsass(
            "@mixin global-mixin() {}\n\na {b: mixin-exists(global_mixin)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
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
            "@mixin global-mixin() {}\n\na {b: mixin-exists(global-mixin)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
    }
    #[test]
    fn keyword() {
        assert_eq!(
            rsass("a {b: mixin-exists($name: foo)}\n").unwrap(),
            "a {\n  b: false;\n}\n"
        );
    }
    #[test]
    fn local() {
        assert_eq!(
        rsass(
            "a {\n  @mixin local-mixin() {}\n  b: mixin-exists(local-mixin);\n}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
    }
    #[test]
    fn non_existent() {
        assert_eq!(
            rsass("a {\n  b: mixin-exists(non-existent);\n}\n").unwrap(),
            "a {\n  b: false;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn through_import() {
        assert_eq!(
            rsass("@import \"other\";\na {b: mixin-exists(global-mixin)}\n")
                .unwrap(),
            "a {\n  b: true;\n}\n"
        );
    }
}

mod string;

// From "sass-spec/spec/core_functions/variable_exists.hrx"
mod variable_exists {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn dash_insensitive() {
        assert_eq!(
        rsass(
            "$global-variable: null;\n\na {b: variable-exists(global_variable)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
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
            "$global-variable: null;\n\na {b: variable-exists(global-variable)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
    }
    #[test]
    fn keyword() {
        assert_eq!(
            rsass("a {b: variable-exists($name: foo)}\n").unwrap(),
            "a {\n  b: false;\n}\n"
        );
    }
    #[test]
    fn local() {
        assert_eq!(
        rsass(
            "a {\n  $local-variable: null;\n  b: variable-exists(local-variable);\n}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
    }
    #[test]
    fn non_existent() {
        assert_eq!(
            rsass("a {\n  b: variable-exists(non-existent);\n}\n").unwrap(),
            "a {\n  b: false;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn through_import() {
        assert_eq!(
        rsass(
            "@import \"other\";\na {b: variable-exists(global-variable)}\n"
        )
        .unwrap(),
        "a {\n  b: true;\n}\n"
    );
    }
}

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
