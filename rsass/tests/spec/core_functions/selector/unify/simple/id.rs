//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/simple/id.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("id")
}

#[test]
fn different() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"#c\", \"#d\"))}\n"),
        "a {\
         \n  b: null;\
         \n}\n"
    );
}
#[test]
fn same() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"#c\", \"#c\")}\n"),
        "a {\
         \n  b: #c;\
         \n}\n"
    );
}
