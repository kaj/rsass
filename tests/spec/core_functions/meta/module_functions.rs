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

    // Ignoring "before_load", error tests are not supported yet.

    // Ignoring "dash_sensitive", error tests are not supported yet.

    // Ignoring "global", error tests are not supported yet.

    // Ignoring "missing", error tests are not supported yet.

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
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
