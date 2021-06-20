//! Tests auto-converted from "sass-spec/spec/core_functions/modules/meta.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn call() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.call(meta.get-function(\"rgb\"), 1, 2, 3)}\n"),
        "a {\
         \n  b: #010203;\
         \n}\n"
    );
}
#[test]
fn content_exists() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \n@mixin print-content-exists {\
             \n  a {b: meta.content-exists()}\
             \n}\n\
             \n@include print-content-exists;\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn feature_exists() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.feature-exists(at-error)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn function_exists() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.function-exists(c)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn get_function() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.inspect(meta.get-function(rgb))}\n"),
        "a {\
         \n  b: get-function(\"rgb\");\
         \n}\n"
    );
}
#[test]
fn global_variable_exists() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.global-variable-exists(c)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn inspect() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.inspect(())}\n"),
        "a {\
         \n  b: ();\
         \n}\n"
    );
}
#[test]
fn keywords() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \n@function keywords($args...) {\
             \n  @return meta.keywords($args);\
             \n}\n\
             \na {b: meta.inspect(keywords($c: d))}\n"),
        "a {\
         \n  b: (c: d);\
         \n}\n"
    );
}
#[test]
fn mixin_exists() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.mixin-exists(c)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn type_of() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(())}\n"),
        "a {\
         \n  b: list;\
         \n}\n"
    );
}
#[test]
fn variable_exists() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.variable-exists(c)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
