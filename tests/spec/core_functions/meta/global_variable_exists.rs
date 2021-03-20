//! Tests auto-converted from "sass-spec/spec/core_functions/meta/global_variable_exists.hrx"

mod dash_insensitive {
    #[test]
    fn dash_to_underscore() {
        assert_eq!(
            crate::rsass(
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
            crate::rsass(
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
    #[test]
    #[ignore] // unexepected error
    fn chosen_prefix() {
        assert_eq!(
            crate::rsass(
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
    #[ignore] // unexepected error
    fn defined() {
        assert_eq!(
            crate::rsass(
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
        #[test]
        #[ignore] // unexepected error
        fn test_as() {
            assert_eq!(
                crate::rsass(
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
        #[ignore] // unexepected error
        fn bare() {
            assert_eq!(
                crate::rsass(
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
        #[ignore] // unexepected error
        fn hide() {
            assert_eq!(
                crate::rsass(
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
        #[ignore] // unexepected error
        fn show() {
            assert_eq!(
                crate::rsass(
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
    #[ignore] // unexepected error
    fn through_use() {
        assert_eq!(
            crate::rsass(
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
    fn undefined() {
        assert_eq!(
            crate::rsass(
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
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        crate::rsass(
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
    #[test]
    fn global() {
        assert_eq!(
            crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
            crate::rsass(
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
