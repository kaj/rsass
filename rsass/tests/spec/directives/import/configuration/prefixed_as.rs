//! Tests auto-converted from "sass-spec/spec/directives/import/configuration/prefixed_as.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("prefixed_as")
        .mock_file("_midstream.scss", "@forward \"upstream\" as d-*;\n")
        .mock_file("_upstream.scss", "$a: original !default;\nb {c: $a}\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$d-a: configured;\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
