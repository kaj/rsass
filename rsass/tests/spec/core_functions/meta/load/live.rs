//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load/live.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("live")
        .mock_file("_other.scss", "$value: c;\n")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "// `meta.load()` should return a reference to a module that updates as the\
             \n// module changes, not a static copy.\
             \n@use \"sass:meta\";\
             \n@use \"other\";\
             \n$module: meta.load(\"other\");\n\
             \na {\
             \n  before: meta.inspect(meta.module-variables($module));\
             \n  other.$value: b;\
             \n  after: meta.inspect(meta.module-variables($module));\
             \n}\n"
        ),
        "a {\
         \n  before: (\"value\": c);\
         \n  after: (\"value\": b);\
         \n}\n"
    );
}
