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
        #[test]
        fn too_few() {
            assert_eq!(
        crate::rsass(
            "a {b: global-variable-exists()}\
             \n"
        ).unwrap_err(),
        "Error: Missing argument $name.\
         \n  ,--> input.scss\
         \n1 | a {b: global-variable-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function global-variable-exists($name, $module: null) {\
         \n  |           ============================================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn too_many() {
            assert_eq!(
        crate::rsass(
            "a {b: global-variable-exists(c, d, e)}\
             \n"
        ).unwrap_err(),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: global-variable-exists(c, d, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function global-variable-exists($name, $module: null) {\
         \n  |           ============================================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
        }
        mod test_type {
            #[test]
            fn module() {
                assert_eq!(
                    crate::rsass(
                        "a {b: global-variable-exists(\"c\", 1)}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: $module: 1 is not a string.\
         \n  ,\
         \n1 | a {b: global-variable-exists(\"c\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
                );
            }
            #[test]
            fn name() {
                assert_eq!(
                    crate::rsass(
                        "a {b: global-variable-exists(12px)}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: $name: 12px is not a string.\
         \n  ,\
         \n1 | a {b: global-variable-exists(12px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
                );
            }
        }
    }
    #[test]
    #[ignore] // wrong error
    fn conflict() {
        assert_eq!(
            crate::rsass(
                "@use \"other1\" as *;\
             \n@use \"other2\" as *;\
             \n\
             \na {b: global-variable-exists(member)}\
             \n"
            )
            .unwrap_err(),
            "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes variable\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes variable\
         \n... |\
         \n4   | a {b: global-variable-exists(member)}\
         \n    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ variable use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet\
         \n",
        );
    }
    mod module {
        #[test]
        fn built_in_but_not_loaded() {
            assert_eq!(
                crate::rsass(
                    "a {b: global-variable-exists(\"c\", \"color\")}\
             \n"
                )
                .unwrap_err(),
                "Error: There is no module with the namespace \"color\".\
         \n  ,\
         \n1 | a {b: global-variable-exists(\"c\", \"color\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn dash_sensitive() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:color\" as a-b;\
             \nc {d: global-variable-exists(\"c\", $module: \"a_b\")}\
             \n"
                )
                .unwrap_err(),
                "Error: There is no module with the namespace \"a_b\".\
         \n  ,\
         \n2 | c {d: global-variable-exists(\"c\", $module: \"a_b\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn non_existent() {
            assert_eq!(
                crate::rsass(
                    "a {b: global-variable-exists(\"c\", \"d\")}\
             \n"
                )
                .unwrap_err(),
                "Error: There is no module with the namespace \"d\".\
         \n  ,\
         \n1 | a {b: global-variable-exists(\"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
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
