//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/nesting/parent.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("parent")
        .mock_file("end/plain.css", "a {.b& {c: d}}\n")
        .mock_file("mid/plain.css", "a {.b&.c {d: e}}\n")
        .mock_file("only/plain.css", "a {& {b: c}}\n")
        .mock_file("start/plain.css", "a {&.b {c: d}}\n")
}

#[test]
#[ignore] // unexepected error
fn end() {
    let runner = runner().with_cwd("end");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  .b& {\
         \n    c: d;\
         \n  }\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn mid() {
    let runner = runner().with_cwd("mid");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  .b&.c {\
         \n    d: e;\
         \n  }\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn only() {
    let runner = runner().with_cwd("only");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  & {\
         \n    b: c;\
         \n  }\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn start() {
    let runner = runner().with_cwd("start");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  &.b {\
         \n    c: d;\
         \n  }\
         \n}\n"
    );
}
