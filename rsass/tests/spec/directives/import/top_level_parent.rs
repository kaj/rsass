//! Tests auto-converted from "sass-spec/spec/directives/import/top_level_parent.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("top_level_parent")
        .mock_file("top_level_parent/_other.scss", "& {a: b}\n")
}

#[test]
#[ignore] // wrong result
fn top_level_parent() {
    let runner = runner().with_cwd("top_level_parent");
    assert_eq!(
        runner.ok("@import \'other\';\n"),
        "& {\
         \n  a: b;\
         \n}\n"
    );
}
