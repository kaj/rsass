//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/list/comma.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comma")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect((1, 2, 3));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
        "a {\
         \n  value: 1, 2, 3;\
         \n  type: string;\
         \n}\n"
    );
}
