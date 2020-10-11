//! Tests auto-converted from "sass-spec/spec/css/plain/error"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/css/plain/error/expression.hrx"
mod expression {
    #[allow(unused)]
    use super::rsass;
    mod function {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "built_in", error tests are not supported yet.

        // Ignoring "keyword_arguments", error tests are not supported yet.

        // Ignoring "variable_arguments", error tests are not supported yet.
    }
    mod interpolation {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "calc", error tests are not supported yet.

        // Ignoring "identifier", error tests are not supported yet.

        // Ignoring "quoted_string", error tests are not supported yet.

        // Ignoring "standalone", error tests are not supported yet.
    }
    mod list {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "empty", error tests are not supported yet.

        // Ignoring "empty_comma", error tests are not supported yet.
    }

    // Ignoring "map", error tests are not supported yet.
    mod operation {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "addition", error tests are not supported yet.

        // Ignoring "equals", error tests are not supported yet.

        // Ignoring "greater_than", error tests are not supported yet.

        // Ignoring "greater_than_or_equal", error tests are not supported yet.

        // Ignoring "less_than", error tests are not supported yet.

        // Ignoring "less_than_or_equal", error tests are not supported yet.

        // Ignoring "modulo", error tests are not supported yet.

        // Ignoring "multiplication", error tests are not supported yet.

        // Ignoring "not_equals", error tests are not supported yet.

        // Ignoring "subtraction", error tests are not supported yet.
    }

    // Ignoring "parent_selector", error tests are not supported yet.

    // Ignoring "parentheses", error tests are not supported yet.

    // Ignoring "silent_comment", error tests are not supported yet.
    mod variable {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "declaration", error tests are not supported yet.

        // Ignoring "test_use", error tests are not supported yet.
    }
}

// From "sass-spec/spec/css/plain/error/statement.hrx"
mod statement {
    #[allow(unused)]
    use super::rsass;
    mod at_rule {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "at_root", error tests are not supported yet.

        // Ignoring "content", error tests are not supported yet.

        // Ignoring "debug", error tests are not supported yet.

        // Ignoring "each", error tests are not supported yet.

        // Ignoring "error", error tests are not supported yet.

        // Ignoring "extend", error tests are not supported yet.

        // Ignoring "test_for", error tests are not supported yet.

        // Ignoring "function", error tests are not supported yet.

        // Ignoring "test_if", error tests are not supported yet.
        mod import {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "interpolated", error tests are not supported yet.

            // Ignoring "multi", error tests are not supported yet.
            #[test]
            #[ignore] // unexepected error
            fn nested() {
                assert_eq!(
                    rsass("@import \'plain\'").unwrap(),
                    "a {\
        \n  @import \"foo\";\
        \n}\
        \n"
                );
            }
        }

        // Ignoring "include", error tests are not supported yet.

        // Ignoring "interpolation", error tests are not supported yet.

        // Ignoring "mixin", error tests are not supported yet.

        // Ignoring "return", error tests are not supported yet.

        // Ignoring "warn", error tests are not supported yet.

        // Ignoring "test_while", error tests are not supported yet.
    }

    // Ignoring "silent_comment", error tests are not supported yet.
    mod style_rule {
        #[allow(unused)]
        use super::rsass;
        mod interpolation {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "custom_property", error tests are not supported yet.

            // Ignoring "declaration", error tests are not supported yet.

            // Ignoring "selector", error tests are not supported yet.
        }

        // Ignoring "nested", error tests are not supported yet.

        // Ignoring "nested_property", error tests are not supported yet.

        // Ignoring "parent_selector", error tests are not supported yet.

        // Ignoring "placeholder_selector", error tests are not supported yet.
    }
}
