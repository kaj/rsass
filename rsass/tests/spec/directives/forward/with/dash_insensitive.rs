//! Tests auto-converted from "sass-spec/spec/directives/forward/with/dash_insensitive.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("dash_insensitive")
        .mock_file(
            "_midstream.scss",
            "@forward \"upstream\" with ($a_b: configured);\n",
        )
        .mock_file(
            "_upstream.scss",
            "$a-b: original !default;\nb {c: $a-b}\n",
        )
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
