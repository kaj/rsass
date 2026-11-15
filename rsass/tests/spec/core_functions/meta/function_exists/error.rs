//! Tests auto-converted from "sass-spec/spec/core_functions/meta/function_exists/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("error")
        .mock_file(
            "conflict/other1.scss",
            "@function member() {@return from other1}\n",
        )
        .mock_file(
            "conflict/other2.scss",
            "@function member() {@return from other2}\n",
        )
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
             \na {b: meta.function-exists()}\n"
            ),
            "Error: Missing argument $name.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.function-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function function-exists($name, $module: null) {\
         \n  |           ===================================== declaration\
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
             \na {b: meta.function-exists(c, d, e)}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.function-exists(c, d, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function function-exists($name, $module: null) {\
         \n  |           ===================================== declaration\
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
             \na {b: meta.function-exists(\"red\", 1)}\n"
                ),
                "Error: $module: 1 is neither a string nor a module reference.\
         \n  ,\
         \n2 | a {b: meta.function-exists(\"red\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: meta.function-exists(12px)}\n"
                ),
                "Error: $name: 12px is not a string.\
         \n  ,\
         \n2 | a {b: meta.function-exists(12px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: meta.function-exists(member)}\n"
        ),
        "Error: This function is available from multiple global modules.\
         \n    ,\
         \n2   | @use \"other1\" as *;\
         \n    | ================== includes function\
         \n3   | @use \"other2\" as *;\
         \n    | ================== includes function\
         \n... |\
         \n5   | a {b: meta.function-exists(member)}\
         \n    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function use\
         \n    \'\
         \n  input.scss 5:7  root stylesheet",
    );
}
mod module {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("module")
    }

    #[test]
    #[ignore] // wrong error
    fn built_in_but_not_loaded() {
        let runner = runner().with_cwd("built_in_but_not_loaded");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \na {b: meta.function-exists(\"red\", \"color\")}\n"
            ),
            "Error: There is no module with namespace \"color\".\
         \n  ,\
         \n2 | a {b: meta.function-exists(\"red\", \"color\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
                "@use \"sass:meta\";\
             \n@use \"sass:color\" as a-b;\
             \nc {d: meta.function-exists(\"c\", $module: \"a_b\")}\n"
            ),
            "Error: There is no module with namespace \"a_b\".\
         \n  ,\
         \n3 | c {d: meta.function-exists(\"c\", $module: \"a_b\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: meta.function-exists(\"c\", \"d\")}\n"
            ),
            "Error: There is no module with namespace \"d\".\
         \n  ,\
         \n2 | a {b: meta.function-exists(\"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
