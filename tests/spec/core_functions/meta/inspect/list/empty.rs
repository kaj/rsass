//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/list/empty.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("empty")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$result: inspect(());\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: ();\
         \n  type: string;\
         \n}\n"
    );
}
