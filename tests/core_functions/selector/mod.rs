//! Tests auto-converted from "sass-spec/spec/core_functions/selector"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/selector/append.hrx"
mod append {
    #[allow(unused)]
    use super::rsass;
    mod classes {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn double() {
            assert_eq!(
                rsass("a {b: selector-append(\".c, .d\", \".e, .f\")}\n")
                    .unwrap(),
                "a {\n  b: .c.e, .d.e, .c.f, .d.f;\n}\n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass("a {b: selector-append(\".c\", \".d\")}\n").unwrap(),
                "a {\n  b: .c.d;\n}\n"
            );
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "invalid", error tests are not supported yet.

        // Ignoring "leading_combinator", error tests are not supported yet.

        // Ignoring "namespace", error tests are not supported yet.

        // Ignoring "parent", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "universal", error tests are not supported yet.
    }
    #[test]
    #[ignore] // failing
    fn input() {
        assert_eq!(
        rsass(
            "// The full set of possible input formats is tested with `selector-parse()`;\n// this spec just verifies one example for `selector-append()`.\na {b: selector-append((c, d e), (f, g h))}\n"
        )
        .unwrap(),
        "a {\n  b: cf, d ef, cg h, d eg h;\n}\n"
    );
    }
    #[test]
    fn many_args() {
        assert_eq!(
            rsass("a {b: selector-append(\".c\", \".d\", \".e\")}\n")
                .unwrap(),
            "a {\n  b: .c.d.e;\n}\n"
        );
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            rsass("a {b: selector-append(\".c.d\")}\n").unwrap(),
            "a {\n  b: .c.d;\n}\n"
        );
    }
    mod suffix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn descendant() {
            assert_eq!(
                rsass("a {b: selector-append(\"c d\", \"e f\")}\n").unwrap(),
                "a {\n  b: c de f;\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn multiple() {
            assert_eq!(
                rsass("a {b: selector-append(\".c, .d\", \"e, f\")}\n")
                    .unwrap(),
                "a {\n  b: .ce, .de, .cf, .df;\n}\n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass("a {b: selector-append(\".c\", \"d\")}\n").unwrap(),
                "a {\n  b: .cd;\n}\n"
            );
        }
    }
}

mod is_superselector;

mod parse;
