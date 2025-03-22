//! Tests auto-converted from "sass-spec/spec/directives/forward/with/null.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("null")
        .mock_file(
            "_midstream.scss",
            "@forward \"upstream\" with ($a: null);\n",
        )
        .mock_file("_upstream.scss", "$a: original !default;\nb {c: $a}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"midstream\";\n"),
        "b {\
         \n  c: original;\
         \n}\n"
    );
}
