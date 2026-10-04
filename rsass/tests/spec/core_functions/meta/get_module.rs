//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_module.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("get_module")
        .mock_file("equality/user_defined/different/_left.scss", "")
        .mock_file("equality/user_defined/different/_right.scss", "")
        .mock_file("equality/user_defined/same/_other.scss", "")
        .mock_file("meta/inspect/no_namespace/_1.scss", "")
        .mock_file("user_defined/_other.scss", "$other: c;\n")
}

#[test]
#[ignore] // unexepected error
fn built_in() {
    let runner = runner().with_cwd("built_in");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"sass:math\";\
             \na {b: meta.global-variable-exists(\"e\", $module: meta.get-module(\"math\"))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod equality {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("equality")
    }

    mod built_in {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("built_in")
        }

        #[test]
        #[ignore] // unexepected error
        fn different() {
            let runner = runner().with_cwd("different");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"sass:color\";\
             \na {b: meta.get-module(meta) == meta.get-module(color)}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn same() {
            let runner = runner().with_cwd("same");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \na {b: meta.get-module(meta) == meta.get-module(meta)}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn same_value() {
        let runner = runner().with_cwd("same_value");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n$meta: meta.get-module(meta);\
             \na {b: $meta == $meta}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    mod user_defined {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("user_defined")
        }

        #[test]
        #[ignore] // unexepected error
        fn different() {
            let runner = runner().with_cwd("different");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"left\";\
             \n@use \"right\";\
             \na {b: meta.get-module(left) == meta.get-module(right)}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn same() {
            let runner = runner().with_cwd("same");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"other\";\
             \na {b: meta.get-module(other) == meta.get-module(other)}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
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
        #[ignore] // wrong error
        fn too_few() {
            let runner = runner().with_cwd("too_few");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \na {b: meta.get-module()}\n"
                ),
                "Error: Missing argument $module.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.get-module()}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function get-module($module) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn too_many() {
            let runner = runner().with_cwd("too_many");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \na {b: meta.get-module(c, d)}\n"
                ),
                "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.get-module(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function get-module($module) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        mod test_type {
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("type")
            }

            #[test]
            #[ignore] // wrong error
            fn module() {
                let runner = runner().with_cwd("module");
                assert_eq!(
                    runner.err(
                        "@use \"sass:meta\";\
             \na {b: meta.get-module(null)}\n"
                    ),
                    "Error: $module: null is neither a string nor a module reference.\
         \n  ,\
         \n2 | a {b: meta.get-module(null)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
                );
            }
        }
    }
    #[test]
    #[ignore] // wrong error
    fn built_in_but_not_loaded() {
        let runner = runner().with_cwd("built_in_but_not_loaded");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \na {b: meta.get-module(\"color\")}\n"
            ),
            "Error: There is no module with namespace \"color\".\
         \n  ,\
         \n2 | a {b: meta.get-module(\"color\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn dash_sensitive() {
        let runner = runner().with_cwd("dash_sensitive");
        assert_eq!(
            runner.err(
                "@use \"sass:color\" as a-b;\
             \n@use \"sass:meta\";\
             \nc {d: meta.get-module(\"a_b\")}\n"
            ),
            "Error: There is no module with namespace \"a_b\".\
         \n  ,\
         \n3 | c {d: meta.get-module(\"a_b\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn non_existent() {
        let runner = runner().with_cwd("non_existent");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \na {b: meta.get-module(does-not-exist)}\n"
            ),
            "Error: There is no module with namespace \"does-not-exist\".\
         \n  ,\
         \n2 | a {b: meta.get-module(does-not-exist)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod meta {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("meta")
    }

    mod inspect {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("inspect")
        }

        #[test]
        #[ignore] // unexepected error
        fn namespace() {
            let runner = runner().with_cwd("namespace");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \na {b: meta.inspect(meta.get-module(meta))};\n"),
                "a {\
         \n  b: get-module(\"meta\");\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn no_namespace() {
            let runner = runner().with_cwd("no_namespace");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"1\" as one;\
             \na {b: meta.inspect(meta.get-module(one))};\n"),
                "a {\
         \n  b: get-module();\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn type_of() {
        let runner = runner().with_cwd("type_of");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \na {b: meta.type-of(meta.get-module(meta))};\n"),
            "a {\
         \n  b: module;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"sass:math\";\n\
             \na {b: meta.global-variable-exists(\"e\", $module: meta.get-module($module: \"math\"))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn user_defined() {
    let runner = runner().with_cwd("user_defined");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"other\";\
             \na {b: meta.global-variable-exists(\"other\", $module: meta.get-module(\"other\"))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
