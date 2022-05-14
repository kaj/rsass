//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/error.hrx"

#[allow(unused)]
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
        .mock_file(
            "through_forward/hide/_midstream.scss",
            "@forward \"upstream\" hide c;\n",
        )
        .mock_file(
            "through_forward/hide/_upstream.scss",
            "@function c() {@return c}\n",
        )
        .mock_file(
            "through_forward/show/_midstream.scss",
            "@forward \"upstream\" show c;\n",
        )
        .mock_file(
            "through_forward/show/_upstream.scss",
            "@function d() {@return c}\n",
        )
}

mod argument {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("argument")
    }

    #[test]
    fn function_ref() {
        let runner = runner().with_cwd("function_ref");
        assert_eq!(
            runner.err(
                "@function foo() {\
             \n  @return null;\
             \n}\n\
             \n$foo-ref: get-function(foo);\
             \na {b: get-function($foo-ref)}\n"
            ),
            "Error: $name: get-function(\"foo\") is not a string.\
         \n  ,\
         \n6 | a {b: get-function($foo-ref)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 6:7  root stylesheet",
        );
    }
    #[test]
    fn too_few() {
        let runner = runner().with_cwd("too_few");
        assert_eq!(
        runner.err(
            "a {b: get-function()}\n"
        ),
        "Error: Missing argument $name.\
         \n  ,--> input.scss\
         \n1 | a {b: get-function()}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function get-function($name, $css: false, $module: null) {\
         \n  |           =============================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_many() {
        let runner = runner().with_cwd("too_many");
        assert_eq!(
        runner.err(
            "a {b: get-function(c, true, d, e)}\n"
        ),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: get-function(c, true, d, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function get-function($name, $css: false, $module: null) {\
         \n  |           =============================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    mod test_type {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("type")
        }

        #[test]
        fn module() {
            let runner = runner().with_cwd("module");
            assert_eq!(
                runner.err("a {b: get-function(c, $module: 1)}\n"),
                "Error: $module: 1 is not a string.\
         \n  ,\
         \n1 | a {b: get-function(c, $module: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn name() {
            let runner = runner().with_cwd("name");
            assert_eq!(
                runner.err("a {b: get-function(2px)}\n"),
                "Error: $name: 2px is not a string.\
         \n  ,\
         \n1 | a {b: get-function(2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
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
            "@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {b: get-function(member)}\n"
        ),
        "Error: This function is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes function\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes function\
         \n... |\
         \n4   | a {b: get-function(member)}\
         \n    |       ^^^^^^^^^^^^^^^^^^^^ function use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn division() {
    let runner = runner().with_cwd("division");
    assert_eq!(
        runner.err("a {b: get-function(rgb) / get-function(lighten)}\n"),
        "Error: get-function(\"rgb\") isn\'t a valid CSS value.\
         \n  ,\
         \n1 | a {b: get-function(rgb) / get-function(lighten)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn function_exists() {
    let runner = runner().with_cwd("function_exists");
    assert_eq!(
        runner.err(
            "@function add-two($v) {\
             \n  @return $v + 2;\
             \n}\n\
             \n$add-two-fn: get-function(add-two);\n\
             \n.error {\
             \n  error: get-function($add-two-fn);\
             \n}\n"
        ),
        "Error: $name: get-function(\"add-two\") is not a string.\
         \n  ,\
         \n8 |   error: get-function($add-two-fn);\
         \n  |          ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 8:10  root stylesheet",
    );
}
mod module {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("module")
    }

    #[test]
    fn and_css() {
        let runner = runner().with_cwd("and_css");
        assert_eq!(
        runner.err(
            "@use \"sass:color\";\
             \na {b: get-function(\"red\", $css: true, $module: \"color\")}\n"
        ),
        "Error: $css and $module may not both be passed at once.\
         \n  ,\
         \n2 | a {b: get-function(\"red\", $css: true, $module: \"color\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn built_in_but_not_loaded() {
        let runner = runner().with_cwd("built_in_but_not_loaded");
        assert_eq!(
            runner.err("a {b: get-function(\"red\", $module: \"color\")}\n"),
            "Error: There is no module with the namespace \"color\".\
         \n  ,\
         \n1 | a {b: get-function(\"red\", $module: \"color\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn dash_sensitive() {
        let runner = runner().with_cwd("dash_sensitive");
        assert_eq!(
            runner.err(
                "@use \"sass:color\" as a-b;\
             \nc {d: get-function(\"c\", $module: \"a_b\")}\n"
            ),
            "Error: There is no module with the namespace \"a_b\".\
         \n  ,\
         \n2 | c {d: get-function(\"c\", $module: \"a_b\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn non_existent() {
        let runner = runner().with_cwd("non_existent");
        assert_eq!(
            runner.err("a {b: get-function(\"c\", $module: \"d\")}\n"),
            "Error: There is no module with the namespace \"d\".\
         \n  ,\
         \n1 | a {b: get-function(\"c\", $module: \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn undefined() {
        let runner = runner().with_cwd("undefined");
        assert_eq!(
            runner.err(
                "@use \"sass:color\";\
             \na {b: get-function(\"c\", $module: \"color\")}\n"
            ),
            "Error: Function not found: \"c\"\
         \n  ,\
         \n2 | a {b: get-function(\"c\", $module: \"color\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn non_existent() {
    let runner = runner().with_cwd("non_existent");
    assert_eq!(
        runner.err("a {b: get-function(does-not-exist)}\n"),
        "Error: Function not found: does-not-exist\
         \n  ,\
         \n1 | a {b: get-function(does-not-exist)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
mod through_forward {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("through_forward")
    }

    #[test]
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
            runner.err(
                "@use \"midstream\" as *;\
             \na {\
             \n  b: call(get-function(c));\
             \n}\n"
            ),
            "Error: Function not found: c\
         \n  ,\
         \n3 |   b: call(get-function(c));\
         \n  |           ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:11  root stylesheet",
        );
    }
    #[test]
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
            runner.err(
                "@use \"midstream\" as *;\
             \na {\
             \n  b: call(get-function(d));\
             \n}\n"
            ),
            "Error: Function not found: d\
         \n  ,\
         \n3 |   b: call(get-function(d));\
         \n  |           ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:11  root stylesheet",
        );
    }
}
