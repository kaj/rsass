//! Tests auto-converted from "sass-spec/spec/core_functions/global/meta.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("meta")
}

#[test]
fn call() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: call(meta.get-function(\"rgb\"), 1, 2, 3)}\n"),
        "a {\
         \n  b: rgb(1, 2, 3);\
         \n}\n"
    );
}
#[test]
fn content_exists() {
    assert_eq!(
        runner().ok("@mixin print-content-exists {\
             \n  a {b: content-exists()}\
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
        runner().ok("a {b: feature-exists(at-error)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn function_exists() {
    assert_eq!(
        runner().ok("a {b: function-exists(c)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn get_function() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.inspect(get-function(rgb))}\n"),
        "a {\
         \n  b: get-function(\"rgb\");\
         \n}\n"
    );
}
#[test]
fn global_variable_exists() {
    assert_eq!(
        runner().ok("a {b: global-variable-exists(c)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn inspect() {
    assert_eq!(
        runner().ok("a {b: inspect(())}\n"),
        "a {\
         \n  b: ();\
         \n}\n"
    );
}
#[test]
fn keywords() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \n@function helper($args...) {\
             \n  @return keywords($args);\
             \n}\n\
             \na {b: meta.inspect(helper($c: d))}\n"),
        "a {\
         \n  b: (c: d);\
         \n}\n"
    );
}
#[test]
fn mixin_exists() {
    assert_eq!(
        runner().ok("a {b: mixin-exists(c)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn type_of() {
    assert_eq!(
        runner().ok("a {b: type-of(())}\n"),
        "a {\
         \n  b: list;\
         \n}\n"
    );
}
#[test]
fn variable_exists() {
    assert_eq!(
        runner().ok("a {b: variable-exists(c)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
