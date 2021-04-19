//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/error.hrx"

mod argument {
    #[test]
    fn function_ref() {
        assert_eq!(
            crate::rsass(
                "@function foo() {\
             \n  @return null;\
             \n}\
             \n\
             \n$foo-ref: get-function(foo);\
             \na {b: get-function($foo-ref)}\
             \n"
            )
            .unwrap_err(),
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
        assert_eq!(
        crate::rsass(
            "a {b: get-function()}\
             \n"
        ).unwrap_err(),
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
        assert_eq!(
        crate::rsass(
            "a {b: get-function(c, true, d, e)}\
             \n"
        ).unwrap_err(),
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
        #[test]
        fn module() {
            assert_eq!(
                crate::rsass(
                    "a {b: get-function(c, $module: 1)}\
             \n"
                )
                .unwrap_err(),
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
            assert_eq!(
                crate::rsass(
                    "a {b: get-function(2px)}\
             \n"
                )
                .unwrap_err(),
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
#[ignore] // wrong error
fn conflict() {
    assert_eq!(
        crate::rsass(
            "@use \"other1\" as *;\
             \n@use \"other2\" as *;\
             \n\
             \na {b: get-function(member)}\
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
         \n4   | a {b: get-function(member)}\
         \n    |       ^^^^^^^^^^^^^^^^^^^^ function use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn division() {
    assert_eq!(
        crate::rsass(
            "a {b: get-function(rgb) / get-function(lighten)}\
             \n"
        )
        .unwrap_err(),
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
    assert_eq!(
        crate::rsass(
            "@function add-two($v) {\
             \n  @return $v + 2;\
             \n}\
             \n\
             \n$add-two-fn: get-function(add-two);\
             \n\
             \n.error {\
             \n  error: get-function($add-two-fn);\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: $name: get-function(\"add-two\") is not a string.\
         \n  ,\
         \n8 |   error: get-function($add-two-fn);\
         \n  |          ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 8:10  root stylesheet",
    );
}
mod module {
    #[test]
    fn and_css() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
             \na {b: get-function(\"red\", $css: true, $module: \"color\")}\
             \n"
            )
            .unwrap_err(),
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
        assert_eq!(
            crate::rsass(
                "a {b: get-function(\"red\", $module: \"color\")}\
             \n"
            )
            .unwrap_err(),
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
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\" as a-b;\
             \nc {d: get-function(\"c\", $module: \"a_b\")}\
             \n"
            )
            .unwrap_err(),
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
        assert_eq!(
            crate::rsass(
                "a {b: get-function(\"c\", $module: \"d\")}\
             \n"
            )
            .unwrap_err(),
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
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
             \na {b: get-function(\"c\", $module: \"color\")}\
             \n"
            )
            .unwrap_err(),
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
    assert_eq!(
        crate::rsass(
            "a {b: get-function(does-not-exist)}\
             \n"
        )
        .unwrap_err(),
        "Error: Function not found: does-not-exist\
         \n  ,\
         \n1 | a {b: get-function(does-not-exist)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
mod through_forward {
    #[test]
    #[ignore] // wrong error
    fn hide() {
        assert_eq!(
            crate::rsass(
                "@use \"midstream\" as *;\
             \na {\
             \n  b: call(get-function(c));\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Function not found: c\
         \n  ,\
         \n3 |   b: call(get-function(c));\
         \n  |           ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:11  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn show() {
        assert_eq!(
            crate::rsass(
                "@use \"midstream\" as *;\
             \na {\
             \n  b: call(get-function(d));\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Function not found: d\
         \n  ,\
         \n3 |   b: call(get-function(d));\
         \n  |           ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:11  root stylesheet",
        );
    }
}
