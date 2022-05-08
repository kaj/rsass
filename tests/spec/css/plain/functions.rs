//! Tests auto-converted from "sass-spec/spec/css/plain/functions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("alpha/plain.css", "a {b: alpha(0.1)}\n")
        .mock_file("defined_elsewhere/plain.css", "c {d: a()}\n")
        .mock_file("grayscale/plain.css", "a {b: grayscale(0.1)}\n")
        .mock_file("hsl/plain.css", "a {b: hsl(0, 100%, 50%)}\n")
        .mock_file("hsla/plain.css", "a {b: hsla(0, 100%, 50%, 0.5)}\n")
        .mock_file("invert/plain.css", "a {b: invert(0.1)}\n")
        .mock_file("rgb/plain.css", "a {b: rgb(10, 20, 30)}\n")
        .mock_file("rgba/plain.css", "a {b: rgba(10, 20, 30, 0.5)}\n")
        .mock_file("saturate/plain.css", "a {b: saturate(0.1)}\n")
}

#[test]
fn alpha() {
    let runner = runner().with_cwd("alpha");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  b: alpha(0.1);\
         \n}\n"
    );
}
#[test]
fn defined_elsewhere() {
    let runner = runner().with_cwd("defined_elsewhere");
    assert_eq!(
        runner.ok("@function a() {@return b}\n\
             \n@import \"plain\";\n"),
        "c {\
         \n  d: a();\
         \n}\n"
    );
}
#[test]
fn grayscale() {
    let runner = runner().with_cwd("grayscale");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  b: grayscale(0.1);\
         \n}\n"
    );
}
#[test]
fn hsl() {
    let runner = runner().with_cwd("hsl");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
    );
}
#[test]
fn hsla() {
    let runner = runner().with_cwd("hsla");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  b: hsla(0, 100%, 50%, 0.5);\
         \n}\n"
    );
}
#[test]
fn invert() {
    let runner = runner().with_cwd("invert");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  b: invert(0.1);\
         \n}\n"
    );
}
#[test]
fn rgb() {
    let runner = runner().with_cwd("rgb");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  b: rgb(10, 20, 30);\
         \n}\n"
    );
}
#[test]
fn rgba() {
    let runner = runner().with_cwd("rgba");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  b: rgba(10, 20, 30, 0.5);\
         \n}\n"
    );
}
#[test]
fn saturate() {
    let runner = runner().with_cwd("saturate");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  b: saturate(0.1);\
         \n}\n"
    );
}
