//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_mixin/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("error")
        .mock_file("conflict/other1.scss", "@mixin member() {a: other1}\n")
        .mock_file("conflict/other2.scss", "@mixin member() {a: other2}\n")
        .mock_file(
            "through_forward/hide/_midstream.scss",
            "@forward \"upstream\" hide c;\n",
        )
        .mock_file(
            "through_forward/hide/_upstream.scss",
            "@mixin c() {a: c}\n",
        )
        .mock_file(
            "through_forward/show/_midstream.scss",
            "@forward \"upstream\" show c;\n",
        )
        .mock_file(
            "through_forward/show/_upstream.scss",
            "@mixin d() {a: c}\n",
        )
}

mod argument {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("argument")
    }

    #[test]
    #[ignore] // wrong error
    fn mixin_ref() {
        let runner = runner().with_cwd("mixin_ref");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@mixin a() {}\n\
             \n$a-ref: meta.get-mixin(a);\
             \na {b: meta.get-mixin($a-ref)}\n"
            ),
            "Error: $name: get-mixin(\"a\") is not a string.\
         \n  ,\
         \n5 | a {b: meta.get-mixin($a-ref)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 5:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_few() {
        let runner = runner().with_cwd("too_few");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \na {b: meta.get-mixin()}\n"
            ),
            "Error: Missing argument $name.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.get-mixin()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function get-mixin($name, $module: null) {\
         \n  |           =============================== declaration\
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
             \na {b: meta.inspect(meta.get-mixin(c, true, d, e))}\n"
            ),
            "Error: Only 2 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.inspect(meta.get-mixin(c, true, d, e))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function get-mixin($name, $module: null) {\
         \n  |           =============================== declaration\
         \n  \'\
         \n  input.scss 2:20  root stylesheet",
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
             \na {b: meta.get-mixin(c, $module: 1)}\n"
                ),
                "Error: $module: 1 is not a string.\
         \n  ,\
         \n2 | a {b: meta.get-mixin(c, $module: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn name() {
            let runner = runner().with_cwd("name");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \na {b: meta.get-mixin(2px)}\n"
                ),
                "Error: $name: 2px is not a string.\
         \n  ,\
         \n2 | a {b: meta.get-mixin(2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
#[test]
#[ignore] // wrong error
fn conflict() {
    let runner = runner().with_cwd("conflict");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {b: meta.get-mixin(member)}\n"
        ),
        "Error: This mixin is available from multiple global modules.\
         \n    ,\
         \n2   | @use \"other1\" as *;\
         \n    | ================== includes mixin\
         \n3   | @use \"other2\" as *;\
         \n    | ================== includes mixin\
         \n... |\
         \n5   | a {b: meta.get-mixin(member)}\
         \n    |       ^^^^^^^^^^^^^^^^^^^^^^ mixin use\
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
             \na {b: meta.get-mixin(\"red\", $module: \"color\")}\n"
            ),
            "Error: There is no module with the namespace \"color\".\
         \n  ,\
         \n2 | a {b: meta.get-mixin(\"red\", $module: \"color\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \nc {d: meta.get-mixin(\"c\", $module: \"a_b\")}\n"
            ),
            "Error: There is no module with the namespace \"a_b\".\
         \n  ,\
         \n3 | c {d: meta.get-mixin(\"c\", $module: \"a_b\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: meta.get-mixin(\"c\", $module: \"d\")}\n"
            ),
            "Error: There is no module with the namespace \"d\".\
         \n  ,\
         \n2 | a {b: meta.get-mixin(\"c\", $module: \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn undefined() {
        let runner = runner().with_cwd("undefined");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@use \"sass:color\";\
             \na {b: meta.get-mixin(\"c\", $module: \"color\")}\n"
            ),
            "Error: Mixin not found: \"c\"\
         \n  ,\
         \n3 | a {b: meta.get-mixin(\"c\", $module: \"color\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn non_existent() {
    let runner = runner().with_cwd("non_existent");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \na {b: meta.get-mixin(does-not-exist)}\n"
        ),
        "Error: Mixin not found: does-not-exist\
         \n  ,\
         \n2 | a {b: meta.get-mixin(does-not-exist)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod through_forward {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("through_forward")
    }

    #[test]
    #[ignore] // wrong error
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {\
             \n  @include meta.apply(meta.get-mixin(c));\
             \n}\n"
            ),
            "Error: Mixin not found: c\
         \n  ,\
         \n4 |   @include meta.apply(meta.get-mixin(c));\
         \n  |                       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:23  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {\
             \n  @include meta.apply(meta.get-mixin(d));\
             \n}\n"
            ),
            "Error: Mixin not found: d\
         \n  ,\
         \n4 |   @include meta.apply(meta.get-mixin(d));\
         \n  |                       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:23  root stylesheet",
        );
    }
}
