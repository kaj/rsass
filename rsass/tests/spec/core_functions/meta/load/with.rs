//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load/with.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("with")
        .mock_file("_other.scss", "$a: e !default;\n")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "// This is a cursory test to verify that with works at all. More thorough tests\
             \n// exist for `meta.load-css()`, because it\'s easier to verify the behavior based\
             \n// on CSS output. `meta.load()` is expected to have the same behavior.\
             \n@use \"sass:meta\";\
             \n$module: meta.load(\"other\", $with: (a: b));\
             \nc {d: meta.inspect(meta.module-variables($module))}\n"
        ),
        "c {\
         \n  d: (\"a\": b);\
         \n}\n"
    );
}
