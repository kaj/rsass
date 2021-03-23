//! Tests auto-converted from "sass-spec/spec/core_functions/modules/meta.hrx"

#[test]
fn call() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \na {b: meta.call(meta.get-function(\"rgb\"), 1, 2, 3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #010203;\
        \n}\
        \n"
    );
}
#[test]
fn content_exists() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n\
            \n@mixin print-content-exists {\
            \n  a {b: meta.content-exists()}\
            \n}\
            \n\
            \n@include print-content-exists;\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
#[test]
fn feature_exists() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \na {b: meta.feature-exists(at-error)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
#[test]
fn function_exists() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \na {b: meta.function-exists(c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
#[test]
fn get_function() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \na {b: meta.inspect(meta.get-function(rgb))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: get-function(\"rgb\");\
        \n}\
        \n"
    );
}
#[test]
fn global_variable_exists() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \na {b: meta.global-variable-exists(c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
#[test]
fn inspect() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \na {b: meta.inspect(())}\
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
fn keywords() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n\
            \n@function keywords($args...) {\
            \n  @return meta.keywords($args);\
            \n}\
            \n\
            \na {b: meta.inspect(keywords($c: d))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: d);\
        \n}\
        \n"
    );
}
#[test]
fn mixin_exists() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \na {b: meta.mixin-exists(c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
#[test]
fn type_of() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \na {b: meta.type-of(())}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: list;\
        \n}\
        \n"
    );
}
#[test]
fn variable_exists() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \na {b: meta.variable-exists(c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
