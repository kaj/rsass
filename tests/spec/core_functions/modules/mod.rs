//! Tests auto-converted from "sass-spec/spec/core_functions/modules"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/modules/color.hrx"
mod color {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn adjust() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.adjust(#abcdef, $red: 10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #b5cdef;\
        \n}\
        \n"
        );
    }
    #[test]
    fn alpha() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.alpha(#abcdef)}\
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
    fn blue() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.blue(#abcdef)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 239;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn change() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.change(#abcdef, $red: 10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #0acdef;\
        \n}\
        \n"
        );
    }
    #[test]
    fn complement() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.complement(#abcdef)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #efcdab;\
        \n}\
        \n"
        );
    }
    mod css_overloads {
        #[allow(unused)]
        use super::rsass;
        mod alpha {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn multi_arg() {
                assert_eq!(
                    rsass(
                        "@use \"sass:color\";\
            \na {b: color.alpha(c=d, e=f, g=h)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: alpha(c=d, e=f, g=h);\
        \n}\
        \n"
                );
            }
            #[test]
            fn one_arg() {
                assert_eq!(
                    rsass(
                        "@use \"sass:color\";\
            \na {b: color.alpha(c=d)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: alpha(c=d);\
        \n}\
        \n"
                );
            }
        }
        #[test]
        fn grayscale() {
            assert_eq!(
                rsass(
                    "@use \"sass:color\";\
            \na {b: color.grayscale(1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: grayscale(1);\
        \n}\
        \n"
            );
        }
        #[test]
        fn invert() {
            assert_eq!(
                rsass(
                    "@use \"sass:color\";\
            \na {b: color.invert(1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: invert(1);\
        \n}\
        \n"
            );
        }
        #[test]
        fn opacity() {
            assert_eq!(
                rsass(
                    "@use \"sass:color\";\
            \na {b: color.opacity(1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: opacity(1);\
        \n}\
        \n"
            );
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "adjust_color", error tests are not supported yet.

        // Ignoring "adjust_hue", error tests are not supported yet.

        // Ignoring "change_color", error tests are not supported yet.

        // Ignoring "darken", error tests are not supported yet.

        // Ignoring "desaturate", error tests are not supported yet.

        // Ignoring "fade_in", error tests are not supported yet.

        // Ignoring "fade_out", error tests are not supported yet.

        // Ignoring "lighten", error tests are not supported yet.

        // Ignoring "opacify", error tests are not supported yet.

        // Ignoring "saturate", error tests are not supported yet.

        // Ignoring "scale_color", error tests are not supported yet.

        // Ignoring "transparentize", error tests are not supported yet.
    }
    #[test]
    fn green() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.green(#abcdef)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 205;\
        \n}\
        \n"
        );
    }
    #[test]
    fn hue() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.hue(#abcdef)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 210deg;\
        \n}\
        \n"
        );
    }
    #[test]
    fn ie_hex_str() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.ie-hex-str(#abcdef)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #FFABCDEF;\
        \n}\
        \n"
        );
    }
    #[test]
    fn invert() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.invert(#abcdef)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #543210;\
        \n}\
        \n"
        );
    }
    #[test]
    fn lightness() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.lightness(#abcdef)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 80.3921568627%;\
        \n}\
        \n"
        );
    }
    #[test]
    fn mix() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.mix(#abcdef, #daddee)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #c3d5ef;\
        \n}\
        \n"
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.red(#abcdef)}\
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
    fn saturation() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.saturation(#abcdef)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 68%;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn scale() {
        assert_eq!(
            rsass(
                "@use \"sass:color\";\
            \na {b: color.scale(#abcdef, $red: 10%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #b3cdef;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/modules/general.hrx"
mod general {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn test_as() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as m;\
            \na {b: m.round(0.7)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "set_variable", error tests are not supported yet.
    }
    mod forward {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn test_as() {
            assert_eq!(
                rsass(
                    "@use \"other\";\
            \na {b: other.s-round(0.7)}\
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
        fn bare() {
            assert_eq!(
                rsass(
                    "@use \"other\";\
            \na {b: other.round(0.7)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1;\
        \n}\
        \n"
            );
        }
        mod error {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "hide", error tests are not supported yet.

            // Ignoring "show", error tests are not supported yet.
        }
        #[test]
        #[ignore] // wrong result
        fn hide() {
            assert_eq!(
                rsass(
                    "@use \"other\";\
            \na {b: other.round(0.7)}\
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
        fn show() {
            assert_eq!(
                rsass(
                    "@use \"other\";\
            \na {b: other.round(0.7)}\
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
    #[ignore] // unexepected error
    fn global() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as *;\
            \na {b: compatible(1px, 1in)}\
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

// From "sass-spec/spec/core_functions/modules/map.hrx"
mod map {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "map_get", error tests are not supported yet.

        // Ignoring "map_has_key", error tests are not supported yet.

        // Ignoring "map_keys", error tests are not supported yet.

        // Ignoring "map_merge", error tests are not supported yet.

        // Ignoring "map_remove", error tests are not supported yet.

        // Ignoring "map_values", error tests are not supported yet.
    }
    #[test]
    fn get() {
        assert_eq!(
            rsass(
                "@use \"sass:map\";\
            \na {b: map.get((c: d), c)}\
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
    fn has_key() {
        assert_eq!(
            rsass(
                "@use \"sass:map\";\
            \na {b: map.has-key((c: d), c)}\
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
    fn keys() {
        assert_eq!(
            rsass(
                "@use \"sass:map\";\
            \na {b: map.keys((c: d))}\
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
    fn merge() {
        assert_eq!(
            rsass(
                "@use \"sass:map\";\
            \n@use \"sass:meta\";\
            \na {b: meta.inspect(map.merge((c: d), (e: f)))}\
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
    fn remove() {
        assert_eq!(
            rsass(
                "@use \"sass:map\";\
            \n@use \"sass:meta\";\
            \na {b: meta.inspect(map.remove((c: d), c))}\
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
    fn values() {
        assert_eq!(
            rsass(
                "@use \"sass:map\";\
            \na {b: map.values((c: d))}\
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

// From "sass-spec/spec/core_functions/modules/math.hrx"
mod math {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn abs() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.abs(-1)}\
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
    fn ceil() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.ceil(0.5)}\
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
    fn compatible() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.compatible(1px, 1in)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "comparable", error tests are not supported yet.

        // Ignoring "unitless", error tests are not supported yet.
    }
    #[test]
    fn floor() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.floor(0.5)}\
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
    fn is_unitless() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.is-unitless(1)}\
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
    fn max() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.max(1, 2, 3)}\
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
    fn min() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.min(1, 2, 3)}\
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
    fn percentage() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.percentage(0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 50%;\
        \n}\
        \n"
        );
    }
    #[test]
    fn random() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.random(5) <= 5}\
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
    fn round() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.round(0.5)}\
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
    fn unit() {
        assert_eq!(
            rsass(
                "@use \"sass:math\";\
            \na {b: math.unit(5px)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"px\";\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/modules/meta.hrx"
mod meta {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn call() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \na {b: meta.call(meta.get-function(\"rgb\"), 1, 2, 3)}\
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
    fn content_exists() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n\
            \n@mixin print-content-exists {\
            \n  a {b: meta.content-exists()}\
            \n}\
            \n\
            \n@include print-content-exists;\
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
    fn feature_exists() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \na {b: meta.feature-exists(at-error)}\
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
    fn function_exists() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \na {b: meta.function-exists(c)}\
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
    fn get_function() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \na {b: meta.inspect(meta.get-function(rgb))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: get-function(\"rgb\");\
        \n}\
        \n"
        );
    }
    #[test]
    fn global_variable_exists() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \na {b: meta.global-variable-exists(c)}\
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
    fn inspect() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \na {b: meta.inspect(())}\
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
    fn keywords() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n\
            \n@function keywords($args...) {\
            \n  @return meta.keywords($args);\
            \n}\
            \n\
            \na {b: meta.inspect(keywords($c: d))}\
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
    fn mixin_exists() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \na {b: meta.mixin-exists(c)}\
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
    fn type_of() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \na {b: meta.type-of(())}\
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
    fn variable_exists() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \na {b: meta.variable-exists(c)}\
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

// From "sass-spec/spec/core_functions/modules/selector.hrx"
mod selector {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn append() {
        assert_eq!(
            rsass(
                "@use \"sass:selector\";\
            \na {b: selector.append(c, d)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: cd;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "selector_append", error tests are not supported yet.

        // Ignoring "selector_extend", error tests are not supported yet.

        // Ignoring "selector_nest", error tests are not supported yet.

        // Ignoring "selector_parse", error tests are not supported yet.

        // Ignoring "selector_replace", error tests are not supported yet.

        // Ignoring "selector_unify", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn extend() {
        assert_eq!(
            rsass(
                "@use \"sass:selector\";\
            \na {b: selector.extend(c, c, d)}\
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
    #[ignore] // wrong result
    fn is_superselector() {
        assert_eq!(
            rsass(
                "@use \"sass:selector\";\
            \na {b: selector.is-superselector(c, d)}\
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
    fn nest() {
        assert_eq!(
            rsass(
                "@use \"sass:selector\";\
            \na {b: selector.nest(c, d)}\
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
    #[ignore] // wrong result
    fn parse() {
        assert_eq!(
            rsass(
                "@use \"sass:selector\";\
            \na {b: selector.parse(\".c, .d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: .c, .d;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn replace() {
        assert_eq!(
            rsass(
                "@use \"sass:selector\";\
            \na {b: selector.replace(c, c, d)}\
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
    #[ignore] // wrong result
    fn simple_selectors() {
        assert_eq!(
            rsass(
                "@use \"sass:selector\";\
            \na {b: selector.simple-selectors(\".c.d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: .c, .d;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn unify() {
        assert_eq!(
            rsass(
                "@use \"sass:selector\";\
            \na {b: selector.unify(\".c\", \".d\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: .c.d;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/modules/string.hrx"
mod string {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "str_index", error tests are not supported yet.

        // Ignoring "str_insert", error tests are not supported yet.

        // Ignoring "str_length", error tests are not supported yet.

        // Ignoring "str_slice", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn index() {
        assert_eq!(
            rsass(
                "@use \"sass:string\";\
            \na {b: string.index(\"c\", \"c\")}\
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
    fn insert() {
        assert_eq!(
            rsass(
                "@use \"sass:string\";\
            \na {b: string.insert(\"c\", \"d\", 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"dc\";\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn length() {
        assert_eq!(
            rsass(
                "@use \"sass:string\";\
            \na {b: string.length(\"c\")}\
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
    fn quote() {
        assert_eq!(
            rsass(
                "@use \"sass:string\";\
            \na {b: string.quote(c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"c\";\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn slice() {
        assert_eq!(
            rsass(
                "@use \"sass:string\";\
            \na {b: string.slice(\"c\", 1, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"c\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn to_upper_case() {
        assert_eq!(
            rsass(
                "@use \"sass:string\";\
            \na {b: string.to-upper-case(\"c\")}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"C\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn unique_id() {
        assert_eq!(
            rsass(
                "@use \"sass:meta\";\
            \n@use \"sass:string\";\
            \na {b: meta.type-of(string.unique-id())}\
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
    fn unquote() {
        assert_eq!(
            rsass(
                "@use \"sass:string\";\
            \na {b: string.unquote(\"c\")}\
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
