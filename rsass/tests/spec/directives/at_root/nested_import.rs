//! Tests auto-converted from "sass-spec/spec/directives/at_root/nested_import.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("nested_import")
        .mock_file(
            "with_builtin_use/other.scss",
            "@use \"sass:math\";\n\n@at-root {\n  b {\n    c: d;\n  }\n}\n",
        )
        .mock_file(
            "with_no_use/other.scss",
            "@at-root {\n  b {\n    c: d;\n  }\n}\n",
        )
        .mock_file(
            "with_user_use/other.scss",
            "@use \"used\";\n\n@at-root {\n  b {\n    c: d;\n  }\n}\n",
        )
        .mock_file("with_user_use/used.scss", "// nothing\n")
}

#[test]
fn with_builtin_use() {
    let runner = runner().with_cwd("with_builtin_use");
    assert_eq!(
        runner.ok("a {\
             \n  @import \"other\";\
             \n}\n"),
        "b {\
         \n  c: d;\
         \n}\n"
    );
}
#[test]
fn with_no_use() {
    let runner = runner().with_cwd("with_no_use");
    assert_eq!(
        runner.ok("a {\
             \n  @import \"other\";\
             \n}\n"),
        "b {\
         \n  c: d;\
         \n}\n"
    );
}
#[test]
fn with_user_use() {
    let runner = runner().with_cwd("with_user_use");
    assert_eq!(
        runner.ok("a {\
             \n  @import \"other\";\
             \n}\n"),
        "b {\
         \n  c: d;\
         \n}\n"
    );
}
