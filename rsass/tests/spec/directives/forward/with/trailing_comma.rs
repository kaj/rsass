//! Tests auto-converted from "sass-spec/spec/directives/forward/with/trailing_comma.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("trailing_comma")
        .mock_file(
            "default/_midstream.scss",
            "@forward \"upstream\" with ($a: configured !default,);\n",
        )
        .mock_file(
            "default/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "single/_midstream.scss",
            "@forward \"upstream\" with ($a: configured,);\n",
        )
        .mock_file(
            "single/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
}

#[test]
fn default() {
    let runner = runner().with_cwd("default");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn single() {
    let runner = runner().with_cwd("single");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
