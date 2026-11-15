//! Tests auto-converted from "sass-spec/spec/core_functions/meta/function_exists/named.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"sass:color\";\n\
             \na {b: meta.function-exists($name: \"red\", $module: \"color\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
