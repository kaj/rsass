//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/plain_css.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("at_rule/_other.scss", "@media screen {a {b: c}}\n")
        .mock_file("empty/user_defined/_other.scss", "// No CSS here!\n")
        .mock_file("named/_other.scss", "a {b: c}\n")
        .mock_file("nested/media_query/_midstream.scss", "@use \"upstream\";\n\na {\n  @media b {c: d}\n}\n")
        .mock_file("nested/media_query/_upstream.scss", "/**/\n")
        .mock_file("nested/parent_selector/_other.scss", "b {\n  // This & should *not* refer to the `a` in `input.scss`, because the CSS is\n  // resolved context-independently.\n  c & {x: y}\n}\n")
        .mock_file("nested/plain_plain_css/_other.scss", "b {c: d}\n")
        .mock_file("plain_css_import/_other.scss", "d {e: f}\n\n// This should be lifted to the top of the output stylesheet.\n@import \"style.css\";\n")
        .mock_file("style_rule/_other.scss", "a {b: c}\n")
        .mock_file("through_other_mixin/_upstream.scss", "a {b: in main dir}\n")
        .mock_file("through_other_mixin/subdir/_midstream.scss", "@use \"sass:meta\";\n\n// This should load relative to _midstream.scss, not input.scss.\n@mixin load-css($module) {\n  @include meta.load-css($module);\n}\n")
        .mock_file("through_other_mixin/subdir/_upstream.scss", "a {b: in subdir}\n")
}

#[test]
fn at_rule() {
    let runner = runner().with_cwd("at_rule");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"),
        "@media screen {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n}\n"
    );
}
mod empty {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("empty")
    }

    #[test]
    fn built_in() {
        let runner = runner().with_cwd("built_in");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"sass:color\");\n\
             \n/* No output other than this */\n"),
            "/* No output other than this */\n"
        );
    }
    #[test]
    fn user_defined() {
        let runner = runner().with_cwd("user_defined");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n\
             \n/* No output other than this */\n"),
            "/* No output other than this */\n"
        );
    }
}
#[test]
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css($url: \"other\");\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
mod nested {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("nested")
    }

    #[test]
    fn media_query() {
        let runner = runner().with_cwd("media_query");
        assert_eq!(
            runner.ok("// Regression test for dart-sass#843\
             \n@use \"sass:meta\";\
             \n@include meta.load-css(\"midstream\")\n"),
            "/**/\
         \n@media b {\
         \n  a {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn parent_selector() {
        let runner = runner().with_cwd("parent_selector");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \na {@include meta.load-css(\"other\")}\n"),
            "a c b {\
         \n  x: y;\
         \n}\n"
        );
    }
    #[test]
    fn plain_plain_css() {
        let runner = runner().with_cwd("plain_plain_css");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \na {@include meta.load-css(\"other\")}\n"),
            "a b {\
         \n  c: d;\
         \n}\n"
        );
    }
}
#[test]
fn plain_css_import() {
    let runner = runner().with_cwd("plain_css_import");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\n\
             \na {b: c}\n\
             \n@include meta.load-css(\"other\");\n"),
        "@import \"style.css\";\
         \na {\
         \n  b: c;\
         \n}\
         \nd {\
         \n  e: f;\
         \n}\n"
    );
}
#[test]
fn style_rule() {
    let runner = runner().with_cwd("style_rule");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn through_other_mixin() {
    let runner = runner().with_cwd("through_other_mixin");
    assert_eq!(
        runner.ok("@use \"subdir/midstream\";\
             \n@include midstream.load-css(\"upstream\");\n"),
        "a {\
         \n  b: in subdir;\
         \n}\n"
    );
}
