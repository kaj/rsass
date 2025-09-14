//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/single.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("single")
        .mock_file("_other.scss", "$a: original !default;\nb {c: $a}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: configured));\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
