//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/nesting/with_declaration.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("with_declaration")
        .mock_file("after/plain.css", "a {\n  b {c: d}\n  e: f;\n}\n")
        .mock_file("before/plain.css", "a {\n  b: c;\n  d {e: f}\n}\n")
        .mock_file("both/plain.css", "a {\n  b: c;\n  d {e: f}\n  g: h;\n}\n")
}

#[test]
#[ignore] // unexepected error
fn after() {
    let runner = runner().with_cwd("after");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  b {\
         \n    c: d;\
         \n  }\
         \n  e: f;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn before() {
    let runner = runner().with_cwd("before");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  b: c;\
         \n  d {\
         \n    e: f;\
         \n  }\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn both() {
    let runner = runner().with_cwd("both");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  b: c;\
         \n  d {\
         \n    e: f;\
         \n  }\
         \n  g: h;\
         \n}\n"
    );
}
