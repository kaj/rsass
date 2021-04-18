//! Tests auto-converted from "sass-spec/spec/core_functions/meta/module_variables.hrx"

#[test]
#[ignore] // unexepected error
fn test_as() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"other\" as a;\
            \n\
            \nb {c: meta.inspect(meta.module-variables(\"a\"))}\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: (\"d\": d value, \"e\": e value, \"f\": f value);\
        \n}\
        \n"
    );
}
#[test]
fn core_module() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"meta\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: ();\
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
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"other\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (\"c-d\": c-d value, \"e-f\": e_f value);\
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
            \na {b: meta.inspect(meta.module-variables(\"other\"))}\
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
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"other\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
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
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables($module: \"other\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
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
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"c-d\": d value, \"c-e\": e value, \"c-f\": f value);\
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
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
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
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"e\": e value);\
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
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"c\": c value, \"d\": d value);\
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
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
        \n}\
        \n"
    );
}
