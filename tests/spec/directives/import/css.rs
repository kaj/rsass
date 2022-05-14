//! Tests auto-converted from "sass-spec/spec/directives/import/css.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("css")
        .mock_file(
            "css_import_after_style_rule/import.scss",
            "@use \"sass:math\";\n@import url(http://example.com);\n",
        )
        .mock_file("css_import_after_style_rule/rule.scss", "a {b: c}\n")
}

#[test]
fn css_import_after_style_rule() {
    let runner = runner().with_cwd("css_import_after_style_rule");
    assert_eq!(
        runner.ok("// Regression test for sass/dart-sass#1628.\
             \n@import \"rule\";\
             \n@import \"import\";\n"),
        "@import url(http://example.com);\
         \na {\
         \n  b: c;\
         \n}\n"
    );
}
