//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/mixin.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin")
}

#[test]
#[ignore] // unexepected error
fn builtin() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.inspect(meta.get-mixin(load-css, meta))};\n"),
        "a {\
         \n  b: get-mixin(\"load-css\");\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn user_defined() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@mixin a() {}\
             \na {b: meta.inspect(meta.get-mixin(a))};\n"),
        "a {\
         \n  b: get-mixin(\"a\");\
         \n}\n"
    );
}
