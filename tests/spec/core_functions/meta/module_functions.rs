//! Tests auto-converted from "sass-spec/spec/core_functions/meta/module_functions.hrx"

#[test]
#[ignore] // unexepected error
fn test_as() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"../util\";\
            \n@use \"other\" as b;\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"b\"))\
            \n"
        )
        .unwrap(),
        "a {\
        \n  c: c value;\
        \n  d: d value;\
        \n  e: e value;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn core_module() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \n@use \"sass:meta\";\
            \n\
            \n// We don\'t want to print every function name in this module, since that would\
            \n// make this test brittle when new functions are added. Instead we just test\
            \n// that a couple functions work.\
            \n\
            \n$functions: meta.module-functions(\"meta\");\
            \na {\
            \n  variable-exists: meta.call(map.get($functions, \"variable-exists\"), \"functions\");\
            \n  inspect: meta.call(map.get($functions, \"inspect\"), ());\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  variable-exists: true;\
        \n  inspect: ();\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn dash_sensitive() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"../util\";\
            \n@use \"other\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"other\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b-c: b-c value;\
        \n  d-e: d_e value;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn empty() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-functions(\"other\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: ();\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn before_load() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n\
             \n$a: meta.module-functions(\"other\");\
             \n\
             \n@use \"other\";\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n3 | $a: meta.module-functions(\"other\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn dash_sensitive() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@use \"other-module\";\
             \n\
             \n$a: meta.module-functions(\"other_module\");\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with namespace \"other_module\".\
         \n  ,\
         \n4 | $a: meta.module-functions(\"other_module\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn global() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@use \"other\" as *;\
             \n\
             \n$a: meta.module-functions(\"other\");\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n4 | $a: meta.module-functions(\"other\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:5  root stylesheet",
        );
    }
    #[test]
    fn missing() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n$a: meta.module-functions(\"other\");\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n2 | $a: meta.module-functions(\"other\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n$a: meta.module-functions();\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $module.\
         \n  ,--> input.scss\
         \n2 | $a: meta.module-functions();\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function module-functions($module) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n$a: meta.module-functions(\"meta\", \"c\");\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | $a: meta.module-functions(\"meta\", \"c\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function module-functions($module) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n$a: meta.module-functions(1);\
             \n"
            )
            .unwrap_err(),
            "Error: $module: 1 is not a string.\
         \n  ,\
         \n2 | $a: meta.module-functions(1);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
}
#[test]
#[ignore] // unexepected error
fn multiple() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"../util\";\
            \n@use \"other\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"other\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b value;\
        \n  c: c value;\
        \n  d: d value;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"../util\";\
            \n@use \"other\";\
            \n\
            \n@include util.print-function-map(meta.module-functions($module: \"other\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b value;\
        \n  c: c value;\
        \n  d: d value;\
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
            "@use \"sass:meta\";\
            \n@use \"../../util\";\
            \n@use \"used\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"used\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b-c: c value;\
        \n  b-d: d value;\
        \n  b-e: e value;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"../../util\";\
            \n@use \"used\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"used\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b value;\
        \n  c: c value;\
        \n  d: d value;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"../../util\";\
            \n@use \"used\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"used\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  d: d value;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"../../util\";\
            \n@use \"used\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"used\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b value;\
        \n  c: c value;\
        \n}\
        \n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn through_import() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"../util\";\
            \n@use \"used\";\
            \n\
            \n@include util.print-function-map(meta.module-functions(\"used\"));\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b value;\
        \n  c: c value;\
        \n  d: d value;\
        \n}\
        \n"
    );
}
