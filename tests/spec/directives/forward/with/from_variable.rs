//! Tests auto-converted from "sass-spec/spec/directives/forward/with/from_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("from_variable")
        .mock_file(
            "_midstream.scss",
            "$a: configured;\n@forward \"upstream\" with ($a: $a);\n",
        )
        .mock_file("_upstream.scss", "$a: original a !default;\nb {c: $a}\n")
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
