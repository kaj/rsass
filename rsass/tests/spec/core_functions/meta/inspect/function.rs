//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/function.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n$result: meta.inspect(meta.get-function(\"get-function\", $module: \"meta\"));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: get-function(\"get-function\");\
         \n  type: string;\
         \n}\n"
    );
}
