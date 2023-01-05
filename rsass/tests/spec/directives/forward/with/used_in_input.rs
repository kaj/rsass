//! Tests auto-converted from "sass-spec/spec/directives/forward/with/used_in_input.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("used_in_input")
        .mock_file(
            "_midstream.scss",
            "@forward \"upstream\" with ($a: configured);\n",
        )
        .mock_file("_upstream.scss", "$a: original !default;\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"midstream\";\
             \nb {c: midstream.$a}\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
