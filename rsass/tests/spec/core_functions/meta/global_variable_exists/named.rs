//! Tests auto-converted from "sass-spec/spec/core_functions/meta/global_variable_exists/named.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("named")
        .mock_file("_other.scss", "$c: null;\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"other\";\
             \na {b: meta.global-variable-exists($name: \"c\", $module: \"other\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
