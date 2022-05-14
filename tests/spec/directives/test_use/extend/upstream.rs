//! Tests auto-converted from "sass-spec/spec/directives/use/extend/upstream.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("upstream")
        .mock_file(
            "double/_other.scss",
            "upstream {a: b}\ndownstream {@extend upstream}\n",
        )
        .mock_file("far/_midstream.scss", "@use \"upstream\";\n")
        .mock_file("far/_upstream.scss", "in-upstream {x: y}\n")
        .mock_file("near/_other.scss", "in-other {x: y}\n")
        .mock_file("placeholder/_other.scss", "%in-other {x: y}\n")
}

#[test]
#[ignore] // wrong result
fn double() {
    let runner = runner().with_cwd("double");
    assert_eq!(
        runner.ok("// Regression test for sass/dart-sass#1393\
             \n@use \"other\";\n\
             \ndownstream {@extend upstream}\n"),
        "upstream, downstream {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn far() {
    let runner = runner().with_cwd("far");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \nin-input {@extend in-upstream}\n"),
        "in-upstream, in-input {\
         \n  x: y;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn near() {
    let runner = runner().with_cwd("near");
    assert_eq!(
        runner.ok("@use \"other\";\n\
             \nin-input {@extend in-other}\n"),
        "in-other, in-input {\
         \n  x: y;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn placeholder() {
    let runner = runner().with_cwd("placeholder");
    assert_eq!(
        runner.ok("@use \"other\";\n\
             \nin-input {@extend %in-other}\n"),
        "in-input {\
         \n  x: y;\
         \n}\n"
    );
}
