//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/nesting/through_import.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("through_import")
        .mock_file("one_level/plain.css", "b {c: d}\n")
        .mock_file("top_level_parent/plain.css", "& {b {c: d}}\n")
        .mock_file("two_levels/plain.css", "b {c {d: e}}\n")
}

#[test]
fn one_level() {
    let runner = runner().with_cwd("one_level");
    assert_eq!(
        runner.ok("a {@import \"plain\"}\n"),
        "a b {\
         \n  c: d;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn top_level_parent() {
    let runner = runner().with_cwd("top_level_parent");
    assert_eq!(
        runner.ok("a {@import \"plain\"}\n"),
        "a {\
         \n  & {\
         \n    b {\
         \n      c: d;\
         \n    }\
         \n  }\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn two_levels() {
    let runner = runner().with_cwd("two_levels");
    assert_eq!(
        runner.ok("a {@import \"plain\"}\n"),
        "a b {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
    );
}
