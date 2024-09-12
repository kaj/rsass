//! Tests auto-converted from "sass-spec/spec/css/plain/error/statement/at_rule.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("at_rule")
        .mock_file(
            "at_root/plain.css",
            "a {\n  @at-root b {\n    x: y;\n  }\n}\n",
        )
        .mock_file("content/plain.css", "@content;\n")
        .mock_file("debug/plain.css", "@debug foo;\n")
        .mock_file(
            "each/plain.css",
            "@each $i in 1 2 3 {\n  a {\n    x: y;\n  }\n}\n",
        )
        .mock_file("error/plain.css", "@error foo;\n")
        .mock_file("extend/plain.css", "a {\n  @extend b;\n}\n")
        .mock_file(
            "for/plain.css",
            "@for $i from 1 to 5 {\n  a {\n    x: y;\n  }\n}\n",
        )
        .mock_file(
            "function/plain.css",
            "@function foo() {\n  @return 1;\n}\n",
        )
        .mock_file("if/plain.css", "@if true {\n  a {\n    x: y;\n  }\n}\n")
        .mock_file(
            "import/interpolated/plain.css",
            "@import url(\"foo#{bar}baz\");\n",
        )
        .mock_file("import/multi/plain.css", "@import \"foo\", \"bar\";\n")
        .mock_file("import/nested/plain.css", "a {\n  @import \"foo\";\n}\n")
        .mock_file("include/plain.css", "@include foo;\n")
        .mock_file("interpolation/plain.css", "@foo a#{b}c;\n")
        .mock_file(
            "mixin/plain.css",
            "@mixin foo {\n  a {\n    x: y;\n  }\n}\n",
        )
        .mock_file("return/plain.css", "@return foo;\n")
        .mock_file("warn/plain.css", "@warn foo;\n")
        .mock_file(
            "while/plain.css",
            "@while false {\n  a {\n    x: y;\n  }\n}\n",
        )
}

#[test]
#[ignore] // wrong error
fn at_root() {
    let runner = runner().with_cwd("at_root");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   @at-root b {\
         \n  |   ^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:3   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn content() {
    let runner = runner().with_cwd("content");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @content;\
         \n  | ^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn debug() {
    let runner = runner().with_cwd("debug");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @debug foo;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn each() {
    let runner = runner().with_cwd("each");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @each $i in 1 2 3 {\
         \n  | ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn error() {
    let runner = runner().with_cwd("error");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @error foo;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn extend() {
    let runner = runner().with_cwd("extend");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   @extend b;\
         \n  |   ^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:3   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn test_for() {
    let runner = runner().with_cwd("for");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @for $i from 1 to 5 {\
         \n  | ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn function() {
    let runner = runner().with_cwd("function");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @function foo() {\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn test_if() {
    let runner = runner().with_cwd("if");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @if true {\
         \n  | ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
mod import {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("import")
    }

    #[test]
    #[ignore] // wrong error
    fn interpolated() {
        let runner = runner().with_cwd("interpolated");
        assert_eq!(
            runner.err("@use \'plain\'"),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @import url(\"foo#{bar}baz\");\
         \n  |                 ^^^^^^\
         \n  \'\
         \n  plain.css 1:17  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn multi() {
        let runner = runner().with_cwd("multi");
        assert_eq!(
            runner.err("@use \'plain\'"),
            "Error: expected \";\".\
         \n  ,\
         \n1 | @import \"foo\", \"bar\";\
         \n  |              ^\
         \n  \'\
         \n  plain.css 1:14  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    fn nested() {
        let runner = runner().with_cwd("nested");
        assert_eq!(
            runner.ok("@use \'plain\'"),
            "a {\
         \n  @import \"foo\";\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong error
fn include() {
    let runner = runner().with_cwd("include");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @include foo;\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn interpolation() {
    let runner = runner().with_cwd("interpolation");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @foo a#{b}c;\
         \n  |       ^^^^\
         \n  \'\
         \n  plain.css 1:7   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn mixin() {
    let runner = runner().with_cwd("mixin");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @mixin foo {\
         \n  | ^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn test_return() {
    let runner = runner().with_cwd("return");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @return foo;\
         \n  | ^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn warn() {
    let runner = runner().with_cwd("warn");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @warn foo;\
         \n  | ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn test_while() {
    let runner = runner().with_cwd("while");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @while false {\
         \n  | ^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
