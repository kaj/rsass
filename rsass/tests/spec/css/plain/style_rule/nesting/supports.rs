//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/nesting/supports.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("supports")
        .mock_file("interleaved/plain.css", "a {\n  @supports (b: c) {\n    d {\n      @supports (e: f) {\n        g: h;\n      }\n    }\n  }\n}\n")
        .mock_file("one_level/plain.css", "a {@supports (b: c) {d: e}}\n")
        .mock_file("two_levels/plain.css", "a { b {@supports (c: d) {e: f}}}\n")
}

#[test]
#[ignore] // unexepected error
fn interleaved() {
    let runner = runner().with_cwd("interleaved");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "@supports (b: c) {\
         \n  a {\
         \n    d {\
         \n      @supports (e: f) {\
         \n        g: h;\
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
        "@supports (b: c) {\
         \n  a {\
         \n    d: e;\
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
         \n    @supports (c: d) {\
         \n      e: f;\
         \n    }\
         \n  }\
         \n}\n"
    );
}
