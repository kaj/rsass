//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/empty.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("empty")
        .mock_file("_other.scss", "a {b: c}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: ());\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
