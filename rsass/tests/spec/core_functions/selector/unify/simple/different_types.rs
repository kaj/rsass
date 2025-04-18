//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/simple/different_types.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("different_types")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c\", \"#d\")}\n"),
        "a {\
         \n  b: c#d;\
         \n}\n"
    );
}
