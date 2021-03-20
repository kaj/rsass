//! Tests auto-converted from "sass-spec/spec/css/plain/error/statement.hrx"

mod at_rule {

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

        // Ignoring "interpolated", error tests are not supported yet.

        // Ignoring "multi", error tests are not supported yet.
        #[test]
        #[ignore] // wrong result
        fn nested() {
            assert_eq!(
                crate::rsass("@import \'plain\'").unwrap(),
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

    // Ignoring "test_return", error tests are not supported yet.

    // Ignoring "warn", error tests are not supported yet.

    // Ignoring "test_while", error tests are not supported yet.
}

// Ignoring "silent_comment", error tests are not supported yet.
mod style_rule {
    mod interpolation {

        // Ignoring "custom_property", error tests are not supported yet.

        // Ignoring "declaration", error tests are not supported yet.

        // Ignoring "selector", error tests are not supported yet.
    }

    // Ignoring "nested", error tests are not supported yet.

    // Ignoring "nested_property", error tests are not supported yet.

    // Ignoring "parent_selector", error tests are not supported yet.

    // Ignoring "placeholder_selector", error tests are not supported yet.
}
