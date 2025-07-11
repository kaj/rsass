//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("style_rule")
        .mock_file("nesting/combinator/plain.css", "a {+ b {c: d}}\n")
        .mock_file(
            "nesting/multiple_complex/plain.css",
            "a, b {c, d {e: f}}\n",
        )
        .mock_file("nesting/one_level/plain.css", "a {b {c: d}}\n")
        .mock_file("nesting/parent/end/plain.css", "a {.b& {c: d}}\n")
        .mock_file("nesting/parent/mid/plain.css", "a {.b&.c {d: e}}\n")
        .mock_file("nesting/parent/only/plain.css", "a {& {b: c}}\n")
        .mock_file("nesting/parent/start/plain.css", "a {&.b {c: d}}\n")
        .mock_file("nesting/through_import/one_level/plain.css", "b {c: d}\n")
        .mock_file(
            "nesting/through_import/top_level_parent/plain.css",
            "& {b {c: d}}\n",
        )
        .mock_file(
            "nesting/through_import/two_levels/plain.css",
            "b {c {d: e}}\n",
        )
        .mock_file(
            "nesting/through_load_css/one_level/plain.css",
            "b {c: d}\n",
        )
        .mock_file(
            "nesting/through_load_css/top_level_parent/plain.css",
            "& {b {c: d}}\n",
        )
        .mock_file(
            "nesting/through_load_css/two_levels/plain.css",
            "b {c {d: e}}\n",
        )
        .mock_file("nesting/two_levels/plain.css", "a {b {c {d: e}}}\n")
        .mock_file(
            "nesting/with_declaration/after/plain.css",
            "a {\n  b {c: d}\n  e: f;\n}\n",
        )
        .mock_file(
            "nesting/with_declaration/before/plain.css",
            "a {\n  b: c;\n  d {e: f}\n}\n",
        )
        .mock_file(
            "nesting/with_declaration/both/plain.css",
            "a {\n  b: c;\n  d {e: f}\n  g: h;\n}\n",
        )
        .mock_file("top_level_parent/plain.css", "& {a: b}\n")
}

mod nesting {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("nesting")
    }

    #[test]
    #[ignore] // unexepected error
    fn combinator() {
        let runner = runner().with_cwd("combinator");
        assert_eq!(
            runner.ok("@use \"plain\";\n"),
            "a {\
         \n  + b {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn multiple_complex() {
        let runner = runner().with_cwd("multiple_complex");
        assert_eq!(
            runner.ok("@use \"plain\";\n"),
            "a, b {\
         \n  c, d {\
         \n    e: f;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn one_level() {
        let runner = runner().with_cwd("one_level");
        assert_eq!(
            runner.ok("@use \"plain\";\n"),
            "a {\
         \n  b {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
    mod parent {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("parent")
        }

        #[test]
        #[ignore] // unexepected error
        fn end() {
            let runner = runner().with_cwd("end");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "a {\
         \n  .b& {\
         \n    c: d;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn mid() {
            let runner = runner().with_cwd("mid");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "a {\
         \n  .b&.c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn only() {
            let runner = runner().with_cwd("only");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "a {\
         \n  & {\
         \n    b: c;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn start() {
            let runner = runner().with_cwd("start");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "a {\
         \n  &.b {\
         \n    c: d;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod through_import {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("through_import")
        }

        #[test]
        #[ignore] // wrong result
        fn one_level() {
            let runner = runner().with_cwd("one_level");
            assert_eq!(
                runner.ok("a {@import \"plain\"}\n"),
                "a b {\
         \n  c: d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn top_level_parent() {
            let runner = runner().with_cwd("top_level_parent");
            assert_eq!(
                runner.ok("a {@import \"plain\"}\n"),
                "a {\
         \n  & {\
         \n    b {\
         \n      c: d;\
         \n    }\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn two_levels() {
            let runner = runner().with_cwd("two_levels");
            assert_eq!(
                runner.ok("a {@import \"plain\"}\n"),
                "a b {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod through_load_css {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("through_load_css")
        }

        #[test]
        #[ignore] // wrong result
        fn one_level() {
            let runner = runner().with_cwd("one_level");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\n\
             \na {@include meta.load-css(\"plain\")}\n"),
                "a b {\
         \n  c: d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn top_level_parent() {
            let runner = runner().with_cwd("top_level_parent");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\n\
             \na {@include meta.load-css(\"plain\")}\n"),
                "a {\
         \n  & {\
         \n    b {\
         \n      c: d;\
         \n    }\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn two_levels() {
            let runner = runner().with_cwd("two_levels");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\n\
             \na {@include meta.load-css(\"plain\")}\n"),
                "a b {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn two_levels() {
        let runner = runner().with_cwd("two_levels");
        assert_eq!(
            runner.ok("@use \"plain\";\n"),
            "a {\
         \n  b {\
         \n    c {\
         \n      d: e;\
         \n    }\
         \n  }\
         \n}\n"
        );
    }
    mod with_declaration {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("with_declaration")
        }

        #[test]
        #[ignore] // unexepected error
        fn after() {
            let runner = runner().with_cwd("after");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "a {\
         \n  b {\
         \n    c: d;\
         \n  }\
         \n  e: f;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn before() {
            let runner = runner().with_cwd("before");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "a {\
         \n  b: c;\
         \n  d {\
         \n    e: f;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn both() {
            let runner = runner().with_cwd("both");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "a {\
         \n  b: c;\
         \n  d {\
         \n    e: f;\
         \n  }\
         \n  g: h;\
         \n}\n"
            );
        }
    }
}
#[test]
fn top_level_parent() {
    let runner = runner().with_cwd("top_level_parent");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "& {\
         \n  a: b;\
         \n}\n"
    );
}
