//! Tests auto-converted from "sass-spec/spec/directives/at_root/load_css.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("load_css")
        .mock_file("other.scss", "@at-root {\n  b {\n    c: d;\n  }\n}\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \na {\
             \n  @include meta.load-css(\"other\");\
             \n}\n"),
        "a b {\
         \n  c: d;\
         \n}\n"
    );
}
