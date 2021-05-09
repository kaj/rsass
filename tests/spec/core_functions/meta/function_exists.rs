//! Tests auto-converted from "sass-spec/spec/core_functions/meta/function_exists.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file(
            "different_module/through_forward/as/_midstream.scss",
            "@forward \"upstream\" as b-*;\n",
        )
        .mock_file(
            "different_module/through_forward/as/_upstream.scss",
            "@function c() {@return null}\n",
        )
        .mock_file(
            "different_module/through_forward/bare/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "different_module/through_forward/bare/_upstream.scss",
            "@function c() {@return null}\n",
        )
        .mock_file(
            "different_module/through_forward/hide/_midstream.scss",
            "@forward \"upstream\" hide b;\n",
        )
        .mock_file(
            "different_module/through_forward/hide/_upstream.scss",
            "@function b() {@return null}\n@function c() {@return null}\n",
        )
        .mock_file(
            "different_module/through_forward/show/_midstream.scss",
            "@forward \"upstream\" show b;\n",
        )
        .mock_file(
            "different_module/through_forward/show/_upstream.scss",
            "@function b() {@return null}\n@function c() {@return null}\n",
        )
        .mock_file(
            "different_module/through_use/other.scss",
            "@function global-function() {@return null}\n",
        )
        .mock_file(
            "error/conflict/other1.scss",
            "@function member() {@return from other1}\n",
        )
        .mock_file(
            "error/conflict/other2.scss",
            "@function member() {@return from other2}\n",
        )
        .mock_file(
            "same_module/through_import/other.scss",
            "@function global-function() {@return null}\n",
        )
}

mod different_module {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn chosen_prefix() {
        assert_eq!(
            runner().ok("@use \"sass:color\" as a;\
             \nb {c: function-exists(\"red\", \"a\")}\n"),
            "b {\
         \n  c: true;\
         \n}\n"
        );
    }
    #[test]
    fn defined() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: function-exists(\"red\", \"color\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    mod through_forward {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // unexepected error
        fn test_as() {
            assert_eq!(
                runner().ok("@use \"midstream\" as *;\
             \na {\
             \n  with-prefix: function-exists(b-c);\
             \n  without-prefix: function-exists(c);\
             \n}\n"),
                "a {\
         \n  with-prefix: true;\
         \n  without-prefix: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn bare() {
            assert_eq!(
                runner().ok("@use \"midstream\" as *;\
             \na {b: function-exists(c)}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn hide() {
            assert_eq!(
                runner().ok("@use \"midstream\" as *;\
             \na {\
             \n  hidden: function-exists(b);\
             \n  not-hidden: function-exists(c);\
             \n}\n"),
                "a {\
         \n  hidden: false;\
         \n  not-hidden: true;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn show() {
            assert_eq!(
                runner().ok("@use \"midstream\" as *;\
             \na {\
             \n  shown: function-exists(b);\
             \n  not-shown: function-exists(c);\
             \n}\n"),
                "a {\
         \n  shown: true;\
         \n  not-shown: false;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn through_use() {
        assert_eq!(
            runner().ok("@use \"other\" as *;\
             \na {b: function-exists(global-function)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn undefined() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: function-exists(\"c\", \"color\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    use super::runner;
    mod argument {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn too_few() {
            assert_eq!(
                runner().err("a {b: function-exists()}\n"),
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
                runner().err("a {b: function-exists(c, d, e)}\n"),
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
            #[allow(unused)]
            use super::runner;
            #[test]
            fn module() {
                assert_eq!(
                    runner().err("a {b: function-exists(\"red\", 1)}\n"),
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
                    runner().err("a {b: function-exists(12px)}\n"),
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
            runner().err(
                "@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {b: function-exists(member)}\n"
            ),
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
        #[allow(unused)]
        use super::runner;
        #[test]
        fn built_in_but_not_loaded() {
            assert_eq!(
                runner().err("a {b: function-exists(\"red\", \"color\")}\n"),
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
                runner().err(
                    "@use \"sass:color\" as a-b;\
             \nc {d: function-exists(\"c\", $module: \"a_b\")}\n"
                ),
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
                runner().err("a {b: function-exists(\"c\", \"d\")}\n"),
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
        runner().ok("@use \"sass:color\";\n\
             \na {b: function-exists($name: \"red\", $module: \"color\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod same_module {
    #[allow(unused)]
    use super::runner;
    mod dash_insensitive {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn dash_to_underscore() {
            assert_eq!(
                runner().ok("@function a_b() {@return null}\n\
             \nc {d: function-exists(a-b)}\n"),
                "c {\
         \n  d: true;\
         \n}\n"
            );
        }
        #[test]
        fn underscore_to_dash() {
            assert_eq!(
                runner().ok("@function a-b() {@return null}\n\
             \nc {d: function-exists(a_b)}\n"),
                "c {\
         \n  d: true;\
         \n}\n"
            );
        }
    }
    #[test]
    fn global() {
        assert_eq!(
            runner().ok("@function global-function() {@return null}\n\
             \na {b: function-exists(global-function)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn local() {
        assert_eq!(
            runner().ok("a {\
             \n  @function local-function() {@return null}\
             \n  b: function-exists(local-function);\
             \n}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn non_existent() {
        assert_eq!(
            runner().ok("a {\
             \n  b: function-exists(non-existent);\
             \n}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn through_import() {
        assert_eq!(
            runner().ok("@import \"other\";\
             \na {b: function-exists(global-function)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
