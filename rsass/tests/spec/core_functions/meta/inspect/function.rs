//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/function.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$result: inspect(get-function(\"get-function\"));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: get-function(\"get-function\");\
         \n  type: string;\
         \n}\n"
    );
}
