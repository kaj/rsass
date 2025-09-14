//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/multiple.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("multiple")
        .mock_file("_other.scss", "$a: original a !default;\n$b: original b !default;\n$c: original c !default;\n\nd {\n  a: $a;\n  b: $b;\n  c: $c;\n}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (\
             \n  a: configured a,\
             \n  b: configured b,\
             \n  c: configured c\
             \n));\n"),
        "d {\
         \n  a: configured a;\
         \n  b: configured b;\
         \n  c: configured c;\
         \n}\n"
    );
}
