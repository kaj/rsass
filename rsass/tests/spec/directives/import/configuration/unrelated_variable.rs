//! Tests auto-converted from "sass-spec/spec/directives/import/configuration/unrelated_variable.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("unrelated_variable")
        .mock_file("_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("_upstream.scss", "$a: original !default;\nb {c: $a}\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$a: configured;\
             \n$d: other;\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
