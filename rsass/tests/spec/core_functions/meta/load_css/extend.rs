//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/extend.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("extend")
        .mock_file("in_input/after/_other.scss", "a {b: c}\n")
        .mock_file("in_input/before/_other.scss", "a {b: c}\n")
        .mock_file("in_other/after/_other.scss", "d {@extend a !optional}\n")
        .mock_file("in_other/before/_other.scss", "d {@extend a !optional}\n")
        .mock_file(
            "shared_cssless_midstream/_midstream.scss",
            "@use 'upstream';\n",
        )
        .mock_file(
            "shared_cssless_midstream/_target.scss",
            "@use 'midstream';\n\n.target {a: b}\n",
        )
        .mock_file("shared_cssless_midstream/_upstream.scss", "@c;\n")
        .mock_file(
            "shared_cssless_midstream/extender.scss",
            "@use 'target';\n\n.extender {\n  @extend .target;\n}\n",
        )
}

mod in_input {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("in_input")
    }

    #[test]
    #[ignore] // unexepected error
    fn after() {
        let runner = runner().with_cwd("after");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n\
             \nd {@extend a}\n"),
            "a, d {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn before() {
        let runner = runner().with_cwd("before");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\n\
             \nd {@extend a}\
             \n@include meta.load-css(\"other\");\n"),
            "a, d {\
         \n  b: c;\
         \n}\n"
        );
    }
}
mod in_other {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("in_other")
    }

    #[test]
    #[ignore] // unexepected error
    fn after() {
        let runner = runner().with_cwd("after");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\n\
             \n@include meta.load-css(\"other\");\
             \na {b: c}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn before() {
        let runner = runner().with_cwd("before");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\n\
             \na {b: c}\
             \n@include meta.load-css(\"other\");\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn shared_cssless_midstream() {
    let runner = runner().with_cwd("shared_cssless_midstream");
    assert_eq!(
        runner.ok("// Regression test for sass/sass#3322\
             \n@use \'sass:meta\';\
             \n@use \'target\';\n\
             \n@include meta.load-css(\'extender\');\n"),
        "@c;\
         \n.target {\
         \n  a: b;\
         \n}\
         \n@c;\
         \n.target, .extender {\
         \n  a: b;\
         \n}\n"
    );
}
