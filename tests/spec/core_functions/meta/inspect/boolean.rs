//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/boolean.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("boolean")
}

#[test]
fn test_false() {
    assert_eq!(
        runner().ok("$result: inspect(false);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: false;\
         \n  type: string;\
         \n}\n"
    );
}
#[test]
fn test_true() {
    assert_eq!(
        runner().ok("$result: inspect(true);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: true;\
         \n  type: string;\
         \n}\n"
    );
}
