//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/different_module.hrx"

#[test]
fn chosen_prefix() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\" as a;\
            \nb {c: call(get-function(\"red\", $module: \"a\"), #abcdef)}\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: 171;\
        \n}\
        \n"
    );
}
#[test]
fn defined() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: call(get-function(\"red\", $module: \"color\"), #abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 171;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: call(get-function($name: \"red\", $module: \"color\"), #abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 171;\
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
                "@use \"midstream\" as *;\
            \na {\
            \n  b: call(get-function(c-d));\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: d;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        assert_eq!(
            crate::rsass(
                "@use \"midstream\" as *;\
            \na {b: call(get-function(c))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        assert_eq!(
            crate::rsass(
                "@use \"midstream\" as *;\
            \na {\
            \n  b: call(get-function(d));\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: d;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        assert_eq!(
            crate::rsass(
                "@use \"midstream\" as *;\
            \na {\
            \n  b: call(get-function(c));\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn through_use() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" as *;\
            \na {b: call(get-function(add-two), 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 12;\
        \n}\
        \n"
    );
}
