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
        #[test]
        fn too_few() {
            assert_eq!(
                crate::rsass(
                    "a {b: function-exists()}\
             \n"
                )
                .unwrap_err(),
                "Error: Missing argument $name.\
         \n  ,--> input.scss\
         \n1 | a {b: function-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function function-exists($name, $module: null) {\
         \n  |           ===================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_many() {
            assert_eq!(
                crate::rsass(
                    "a {b: function-exists(c, d, e)}\
             \n"
                )
                .unwrap_err(),
                "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: function-exists(c, d, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function function-exists($name, $module: null) {\
         \n  |           ===================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        mod test_type {
            #[test]
            fn module() {
                assert_eq!(
                    crate::rsass(
                        "a {b: function-exists(\"red\", 1)}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: $module: 1 is not a string.\
         \n  ,\
         \n1 | a {b: function-exists(\"red\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
                );
            }
            #[test]
            fn name() {
                assert_eq!(
                    crate::rsass(
                        "a {b: function-exists(12px)}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: $name: 12px is not a string.\
         \n  ,\
         \n1 | a {b: function-exists(12px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
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
             \na {b: function-exists(member)}\
             \n"
            )
            .unwrap_err(),
            "Error: This function is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes function\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes function\
         \n... |\
         \n4   | a {b: function-exists(member)}\
         \n    |       ^^^^^^^^^^^^^^^^^^^^^^^ function use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
        );
    }
    mod module {
        #[test]
        fn built_in_but_not_loaded() {
            assert_eq!(
                crate::rsass(
                    "a {b: function-exists(\"red\", \"color\")}\
             \n"
                )
                .unwrap_err(),
                "Error: There is no module with the namespace \"color\".\
         \n  ,\
         \n1 | a {b: function-exists(\"red\", \"color\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn dash_sensitive() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:color\" as a-b;\
             \nc {d: function-exists(\"c\", $module: \"a_b\")}\
             \n"
                )
                .unwrap_err(),
                "Error: There is no module with the namespace \"a_b\".\
         \n  ,\
         \n2 | c {d: function-exists(\"c\", $module: \"a_b\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn non_existent() {
            assert_eq!(
                crate::rsass(
                    "a {b: function-exists(\"c\", \"d\")}\
             \n"
                )
                .unwrap_err(),
                "Error: There is no module with the namespace \"d\".\
         \n  ,\
         \n1 | a {b: function-exists(\"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
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
