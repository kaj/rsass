//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/doesnt_run_default.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("doesnt_run_default")
        .mock_file("_other.scss", "// This will throw an error if it's evaluated, but it shouldn't be because `$a`\n// already has a value.\n$a: 1px + 1em !default;\nb {c: $a}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: configured));\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
