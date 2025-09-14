//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/dash_insensitive.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("dash_insensitive")
        .mock_file("_other.scss", "$a-b: original !default;\nb {c: $a-b}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a_b: configured));\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
