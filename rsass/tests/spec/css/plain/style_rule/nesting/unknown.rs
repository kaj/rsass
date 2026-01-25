//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/nesting/unknown.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("unknown")
        .mock_file("interleaved/plain.css", "a {\n  @b {\n    c {\n      @d {\n        e: f;\n      }\n    }\n  }\n}\n")
        .mock_file("one_level/plain.css", "a {@b {c: d}}\n")
        .mock_file("two_levels/plain.css", "a { b {@c {d: e}}}\n")
}

#[test]
#[ignore] // unexepected error
fn interleaved() {
    let runner = runner().with_cwd("interleaved");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "@b {\
         \n  a {\
         \n    c {\
         \n      @d {\
         \n        e: f;\
         \n      }\
         \n    }\
         \n  }\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn one_level() {
    let runner = runner().with_cwd("one_level");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "@b {\
         \n  a {\
         \n    c: d;\
         \n  }\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn two_levels() {
    let runner = runner().with_cwd("two_levels");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  b {\
         \n    @c {\
         \n      d: e;\
         \n    }\
         \n  }\
         \n}\n"
    );
}
