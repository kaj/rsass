//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/list/space.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("space")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$result: inspect(1 2 3);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: 1 2 3;\
         \n  type: string;\
         \n}\n"
    );
}
