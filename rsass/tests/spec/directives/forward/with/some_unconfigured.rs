//! Tests auto-converted from "sass-spec/spec/directives/forward/with/some_unconfigured.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("some_unconfigured")
        .mock_file("_midstream.scss", "@forward \"upstream\" with ($a: configured a);\n")
        .mock_file("_upstream.scss", "$a: original a !default;\n$b: original b !default;\n\nc {\n  a: $a;\n  b: $b;\n}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"midstream\";\n"),
        "c {\
         \n  a: configured a;\
         \n  b: original b;\
         \n}\n"
    );
}
