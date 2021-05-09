//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/meta.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn inspect() {
    assert_eq!(
        runner().ok("a {b: inspect(get-function(lighten))};\n"),
        "a {\
         \n  b: get-function(\"lighten\");\
         \n}\n"
    );
}
#[test]
fn type_of() {
    assert_eq!(
        runner().ok("a {b: type-of(get-function(lighten))};\n"),
        "a {\
         \n  b: function;\
         \n}\n"
    );
}
