//! Tests auto-converted from "sass-spec/spec/core_functions/meta/global_variable_exists.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("global_variable_exists")
        .mock_file(
            "different_module/chosen_prefix/_other.scss",
            "$d: null;\n",
        )
        .mock_file("different_module/defined/_other.scss", "$c: null;\n")
        .mock_file(
            "different_module/through_forward/as/_midstream.scss",
            "@forward \"upstream\" as b-*;\n",
        )
        .mock_file(
            "different_module/through_forward/as/_upstream.scss",
            "$c: null;\n",
        )
        .mock_file(
            "different_module/through_forward/bare/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "different_module/through_forward/bare/_upstream.scss",
            "$c: null;\n",
        )
        .mock_file(
            "different_module/through_forward/hide/_midstream.scss",
            "@forward \"upstream\" hide $b;\n",
        )
        .mock_file(
            "different_module/through_forward/hide/_upstream.scss",
            "$b: null;\n$c: null;\n",
        )
        .mock_file(
            "different_module/through_forward/show/_midstream.scss",
            "@forward \"upstream\" show $b;\n",
        )
        .mock_file(
            "different_module/through_forward/show/_upstream.scss",
            "$b: null;\n$c: null;\n",
        )
        .mock_file(
            "different_module/through_use/other.scss",
            "$global-variable: null;\n",
        )
        .mock_file("error/conflict/other1.scss", "$member: from other1;\n")
        .mock_file("error/conflict/other2.scss", "$member: from other2;\n")
        .mock_file("named/_other.scss", "$c: null;\n")
        .mock_file(
            "same_module/through_import/other.scss",
            "$global-variable: null;\n",
        )
}

mod dash_insensitive {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("dash_insensitive")
    }

    #[test]
    fn dash_to_underscore() {
        let runner = runner().with_cwd("dash_to_underscore");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n$a_b: null;\n\
             \nc {d: meta.global-variable-exists(a-b)}\n"),
            "c {\
         \n  d: true;\
         \n}\n"
        );
    }
    #[test]
    fn underscore_to_dash() {
        let runner = runner().with_cwd("underscore_to_dash");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n$a-b: null;\n\
             \nc {d: meta.global-variable-exists(a_b)}\n"),
            "c {\
         \n  d: true;\
         \n}\n"
        );
    }
}
mod different_module {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("different_module")
    }

    #[test]
    fn chosen_prefix() {
        let runner = runner().with_cwd("chosen_prefix");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"other\" as a;\
             \nb {c: meta.global-variable-exists(\"d\", \"a\")}\n"),
            "b {\
         \n  c: true;\
         \n}\n"
        );
    }
    #[test]
    fn defined() {
        let runner = runner().with_cwd("defined");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"other\";\
             \na {b: meta.global-variable-exists(\"c\", \"other\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    mod through_forward {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("through_forward")
        }

        #[test]
        fn test_as() {
            let runner = runner().with_cwd("as");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {\
             \n  with-prefix: meta.global-variable-exists(b-c);\
             \n  without-prefix: meta.global-variable-exists(c);\
             \n}\n"),
                "a {\
         \n  with-prefix: true;\
         \n  without-prefix: false;\
         \n}\n"
            );
        }
        #[test]
        fn bare() {
            let runner = runner().with_cwd("bare");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {b: meta.variable-exists(c)}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn hide() {
            let runner = runner().with_cwd("hide");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {\
             \n  hidden: meta.global-variable-exists(b);\
             \n  not-hidden: meta.global-variable-exists(c);\
             \n}\n"),
                "a {\
         \n  hidden: false;\
         \n  not-hidden: true;\
         \n}\n"
            );
        }
        #[test]
        fn show() {
            let runner = runner().with_cwd("show");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {\
             \n  shown: meta.global-variable-exists(b);\
             \n  not-shown: meta.global-variable-exists(c);\
             \n}\n"),
                "a {\
         \n  shown: true;\
         \n  not-shown: false;\
         \n}\n"
            );
        }
    }
    #[test]
    fn through_use() {
        let runner = runner().with_cwd("through_use");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"other\" as *;\
             \na {b: meta.global-variable-exists(global-variable)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn undefined() {
        let runner = runner().with_cwd("undefined");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"sass:color\";\
             \na {b: meta.global-variable-exists(\"c\", \"color\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
mod error {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    mod argument {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("argument")
        }

        #[test]
        fn too_few() {
            let runner = runner().with_cwd("too_few");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \na {b: meta.global-variable-exists()}\n"
        ),
        "Error: Missing argument $name.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.global-variable-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function global-variable-exists($name, $module: null) {\
         \n  |           ============================================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn too_many() {
            let runner = runner().with_cwd("too_many");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \na {b: meta.global-variable-exists(c, d, e)}\n"
        ),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.global-variable-exists(c, d, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function global-variable-exists($name, $module: null) {\
         \n  |           ============================================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        mod test_type {
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("type")
            }

            #[test]
            fn module() {
                let runner = runner().with_cwd("module");
                assert_eq!(
                    runner.err(
                        "@use \"sass:meta\";\
             \na {b: meta.global-variable-exists(\"c\", 1)}\n"
                    ),
                    "Error: $module: 1 is not a string.\
         \n  ,\
         \n2 | a {b: meta.global-variable-exists(\"c\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
                );
            }
            #[test]
            fn name() {
                let runner = runner().with_cwd("name");
                assert_eq!(
                    runner.err(
                        "@use \"sass:meta\";\
             \na {b: meta.global-variable-exists(12px)}\n"
                    ),
                    "Error: $name: 12px is not a string.\
         \n  ,\
         \n2 | a {b: meta.global-variable-exists(12px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
                );
            }
        }
    }
    #[test]
    #[ignore] // missing error
    fn conflict() {
        let runner = runner().with_cwd("conflict");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {b: meta.global-variable-exists(member)}\n"
            ),
            "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n2   | @use \"other1\" as *;\
         \n    | ================== includes variable\
         \n3   | @use \"other2\" as *;\
         \n    | ================== includes variable\
         \n... |\
         \n5   | a {b: meta.global-variable-exists(member)}\
         \n    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ variable use\
         \n    \'\
         \n  input.scss 5:7  root stylesheet",
        );
    }
    mod module {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("module")
        }

        #[test]
        fn built_in_but_not_loaded() {
            let runner = runner().with_cwd("built_in_but_not_loaded");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \na {b: meta.global-variable-exists(\"c\", \"color\")}\n"
                ),
                "Error: There is no module with the namespace \"color\".\
         \n  ,\
         \n2 | a {b: meta.global-variable-exists(\"c\", \"color\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn dash_sensitive() {
            let runner = runner().with_cwd("dash_sensitive");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \n@use \"sass:color\" as a-b;\
             \nc {d: meta.global-variable-exists(\"c\", $module: \"a_b\")}\n"
                ),
                "Error: There is no module with the namespace \"a_b\".\
         \n  ,\
         \n3 | c {d: meta.global-variable-exists(\"c\", $module: \"a_b\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
        #[test]
        fn non_existent() {
            let runner = runner().with_cwd("non_existent");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \na {b: meta.global-variable-exists(\"c\", \"d\")}\n"
                ),
                "Error: There is no module with the namespace \"d\".\
         \n  ,\
         \n2 | a {b: meta.global-variable-exists(\"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
#[test]
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"other\";\
             \na {b: meta.global-variable-exists($name: \"c\", $module: \"other\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod same_module {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("same_module")
    }

    #[test]
    fn global() {
        let runner = runner().with_cwd("global");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n$global-variable: null;\n\
             \na {b: meta.global-variable-exists(global-variable)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn local() {
        let runner = runner().with_cwd("local");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \na {\
             \n  $local-variable: null;\
             \n  b: meta.global-variable-exists(local-variable);\
             \n}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn non_existent() {
        let runner = runner().with_cwd("non_existent");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \na {\
             \n  b: meta.global-variable-exists(non-existent);\
             \n}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn through_import() {
        let runner = runner().with_cwd("through_import");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@import \"other\";\
             \na {b: meta.global-variable-exists(global-variable)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
