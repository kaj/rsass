//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/some_unconfigured.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("some_unconfigured")
        .mock_file("_other.scss", "$a: original a !default;\n$b: original b !default;\n\nc {\n  a: $a;\n  b: $b;\n}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: configured a));\n"
        ),
        "c {\
         \n  a: configured a;\
         \n  b: original b;\
         \n}\n"
    );
}
