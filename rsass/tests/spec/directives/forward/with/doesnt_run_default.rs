//! Tests auto-converted from "sass-spec/spec/directives/forward/with/doesnt_run_default.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("doesnt_run_default")
        .mock_file("_midstream.scss", "@forward \"upstream\" with ($a: configured);\n")
        .mock_file("_upstream.scss", "// This will throw an error if it's evaluated, but it shouldn't be because `$a`\n// already has a value.\n$a: 1px + 1em !default;\nb {c: $a}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
