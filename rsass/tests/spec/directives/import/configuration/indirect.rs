//! Tests auto-converted from "sass-spec/spec/directives/import/configuration/indirect.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("indirect")
        .mock_file(
            "through_forward/_midstream.import.scss",
            "@forward \"midstream\";\n",
        )
        .mock_file(
            "through_forward/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "through_forward/_upstream.import.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "through_forward/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "through_import/_midstream.scss",
            "@import \"upstream\";\n",
        )
        .mock_file(
            "through_import/_upstream.import.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "through_import/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
}

#[test]
#[ignore] // wrong result
fn through_forward() {
    let runner = runner().with_cwd("through_forward");
    assert_eq!(
        runner.ok("$a: configured;\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn through_import() {
    let runner = runner().with_cwd("through_import");
    assert_eq!(
        runner.ok("$a: configured;\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
