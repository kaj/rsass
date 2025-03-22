//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/null.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("null")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect(null);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
        "a {\
         \n  value: null;\
         \n  type: string;\
         \n}\n"
    );
}
