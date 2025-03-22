//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/meta.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("meta")
}

#[test]
fn inspect() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \na {b: meta.inspect(meta.get-function(lighten))};\n"),
        "a {\
         \n  b: get-function(\"lighten\");\
         \n}\n"
    );
}
#[test]
fn type_of() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \na {b: meta.type-of(meta.get-function(lighten))};\n"),
        "a {\
         \n  b: function;\
         \n}\n"
    );
}
