//! Tests auto-converted from "sass-spec/spec/directives/import/configuration/midstream_definition.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("midstream_definition")
        .mock_file(
            "no_config/_midstream.scss",
            "$a: midstream;\n@forward \"upstream\";\n",
        )
        .mock_file(
            "no_config/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "with_config/_midstream.scss",
            "$a: midstream;\n@forward \"upstream\";\n",
        )
        .mock_file(
            "with_config/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
}

#[test]
fn no_config() {
    let runner = runner().with_cwd("no_config");
    assert_eq!(
        runner.ok("@import \"midstream\";\n"),
        "b {\
         \n  c: original;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn with_config() {
    let runner = runner().with_cwd("with_config");
    assert_eq!(
        runner.ok("$a: configured;\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
