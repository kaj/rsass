//! Tests auto-converted from "sass-spec/spec/core_functions/meta/global_variable_exists/dash_insensitive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("dash_insensitive")
}

#[test]
fn dash_to_underscore() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$a_b: null;\n\
             \nc {d: meta.global-variable-exists(a-b)}\n"),
        "c {\
         \n  d: true;\
         \n}\n"
    );
}
#[test]
fn underscore_to_dash() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$a-b: null;\n\
             \nc {d: meta.global-variable-exists(a_b)}\n"),
        "c {\
         \n  d: true;\
         \n}\n"
    );
}
