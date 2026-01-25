//! Tests auto-converted from "sass-spec/spec/directives/import/configuration/same_file.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("same_file")
        .mock_file("_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("_upstream.scss", "$a: original !default;\nb {c: $a}\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$a: configured;\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
