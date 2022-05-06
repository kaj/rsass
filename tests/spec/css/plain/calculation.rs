//! Tests auto-converted from "sass-spec/spec/css/plain/calculation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("function/plain.css", "a {b: calc(c())}\n")
        .mock_file("operation/plain.css", "a {b: calc(1px + 1%)}\n")
        .mock_file("parentheses/plain.css", "a {b: calc(2 * (1px + 1%))}\n")
        .mock_file("simplified/plain.css", "a {b: calc(1px)}\n")
}

#[test]
fn function() {
    let runner = runner().with_cwd("function");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  b: calc(c());\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn operation() {
    let runner = runner().with_cwd("operation");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  b: calc(1px + 1%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn parentheses() {
    let runner = runner().with_cwd("parentheses");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  b: calc(2 * (1px + 1%));\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simplified() {
    let runner = runner().with_cwd("simplified");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  b: 1px;\
         \n}\n"
    );
}
