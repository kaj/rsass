//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load/no_error.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("no_error")
        .mock_file("extend/_other.scss", "a {@extend missing}\n")
        .mock_file("serialize/_other.scss", "a {b: (c: d)}\n")
}

#[test]
#[ignore] // unexepected error
fn extend() {
    let runner = runner().with_cwd("extend");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n$_: meta.load(\"other\");\n"),
        ""
    );
}
#[test]
#[ignore] // unexepected error
fn serialize() {
    let runner = runner().with_cwd("serialize");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n$_: meta.load(\"other\");\n"),
        ""
    );
}
