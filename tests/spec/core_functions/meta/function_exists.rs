//! Tests auto-converted from "sass-spec/spec/core_functions/meta/function_exists.hrx"

mod different_module {
    #[test]
    fn chosen_prefix() {
        assert_eq!(
            crate::rsass(
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
    fn defined() {
        assert_eq!(
            crate::rsass(
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
        #[test]
        #[ignore] // unexepected error
        fn test_as() {
            assert_eq!(
                crate::rsass(
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
        #[ignore] // unexepected error
        fn bare() {
            assert_eq!(
                crate::rsass(
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
        #[ignore] // unexepected error
        fn hide() {
            assert_eq!(
                crate::rsass(
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
        #[ignore] // unexepected error
        fn show() {
            assert_eq!(
                crate::rsass(
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
    #[ignore] // unexepected error
    fn through_use() {
        assert_eq!(
            crate::rsass(
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
    fn undefined() {
        assert_eq!(
            crate::rsass(
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
    mod argument {

        // Ignoring "too_few", error tests are not supported yet.

        // Ignoring "too_many", error tests are not supported yet.
        mod test_type {

            // Ignoring "module", error tests are not supported yet.

            // Ignoring "name", error tests are not supported yet.
        }
    }

    // Ignoring "conflict", error tests are not supported yet.
    mod module {

        // Ignoring "built_in_but_not_loaded", error tests are not supported yet.

        // Ignoring "dash_sensitive", error tests are not supported yet.

        // Ignoring "non_existent", error tests are not supported yet.
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
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
    mod dash_insensitive {
        #[test]
        fn dash_to_underscore() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
