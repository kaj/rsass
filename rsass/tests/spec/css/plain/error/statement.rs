//! Tests auto-converted from "sass-spec/spec/css/plain/error/statement.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("statement")
        .mock_file(
            "at_rule/at_root/plain.css",
            "a {\n  @at-root b {\n    x: y;\n  }\n}\n",
        )
        .mock_file("at_rule/content/plain.css", "@content;\n")
        .mock_file("at_rule/debug/plain.css", "@debug foo;\n")
        .mock_file(
            "at_rule/each/plain.css",
            "@each $i in 1 2 3 {\n  a {\n    x: y;\n  }\n}\n",
        )
        .mock_file("at_rule/error/plain.css", "@error foo;\n")
        .mock_file("at_rule/extend/plain.css", "a {\n  @extend b;\n}\n")
        .mock_file(
            "at_rule/for/plain.css",
            "@for $i from 1 to 5 {\n  a {\n    x: y;\n  }\n}\n",
        )
        .mock_file(
            "at_rule/function/plain.css",
            "@function foo() {\n  @return 1;\n}\n",
        )
        .mock_file(
            "at_rule/if/plain.css",
            "@if true {\n  a {\n    x: y;\n  }\n}\n",
        )
        .mock_file(
            "at_rule/import/interpolated/plain.css",
            "@import url(\"foo#{bar}baz\");\n",
        )
        .mock_file(
            "at_rule/import/multi/plain.css",
            "@import \"foo\", \"bar\";\n",
        )
        .mock_file(
            "at_rule/import/nested/plain.css",
            "a {\n  @import \"foo\";\n}\n",
        )
        .mock_file("at_rule/include/plain.css", "@include foo;\n")
        .mock_file("at_rule/interpolation/plain.css", "@foo a#{b}c;\n")
        .mock_file(
            "at_rule/mixin/plain.css",
            "@mixin foo {\n  a {\n    x: y;\n  } \n}\n",
        )
        .mock_file("at_rule/return/plain.css", "@return foo;\n")
        .mock_file("at_rule/warn/plain.css", "@warn foo;\n")
        .mock_file(
            "at_rule/while/plain.css",
            "@while false {\n  a {\n    x: y;\n  }\n}\n",
        )
        .mock_file("silent_comment/plain.css", "// silent\n")
        .mock_file(
            "style_rule/interpolation/custom_property/plain.css",
            "a {\n  --b: #{c};\n}\n",
        )
        .mock_file(
            "style_rule/interpolation/declaration/plain.css",
            "a {\n  w#{x}y: z;\n}\n",
        )
        .mock_file(
            "style_rule/interpolation/selector/plain.css",
            "a#{b}c {\n  x: y;\n}\n",
        )
        .mock_file(
            "style_rule/nested/plain.css",
            "a {\n  b {\n    x: y;\n  }\n}\n",
        )
        .mock_file(
            "style_rule/nested_property/plain.css",
            "a {\n  x: {\n    y: z;\n  }\n}\n",
        )
        .mock_file(
            "style_rule/parent_selector/plain.css",
            "&.foo {\n  x: y\n}\n",
        )
        .mock_file(
            "style_rule/placeholder_selector/plain.css",
            "%foo {\n  x: y;\n}\n",
        )
}

mod at_rule {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("at_rule")
    }

    #[test]
    #[ignore] // wrong error
    fn at_root() {
        let runner = runner().with_cwd("at_root");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   @at-root b {\
         \n  |   ^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:3   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn content() {
        let runner = runner().with_cwd("content");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @content;\
         \n  | ^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn debug() {
        let runner = runner().with_cwd("debug");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @debug foo;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn each() {
        let runner = runner().with_cwd("each");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @each $i in 1 2 3 {\
         \n  | ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn error() {
        let runner = runner().with_cwd("error");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @error foo;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn extend() {
        let runner = runner().with_cwd("extend");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   @extend b;\
         \n  |   ^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:3   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_for() {
        let runner = runner().with_cwd("for");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @for $i from 1 to 5 {\
         \n  | ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @function foo() {\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_if() {
        let runner = runner().with_cwd("if");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @if true {\
         \n  | ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
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
                runner.err("@import \'plain\'"),
                "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @import url(\"foo#{bar}baz\");\
         \n  |                 ^^^^^^\
         \n  \'\
         \n  plain.css 1:17  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn multi() {
            let runner = runner().with_cwd("multi");
            assert_eq!(
                runner.err("@import \'plain\'"),
                "Error: expected \";\".\
         \n  ,\
         \n1 | @import \"foo\", \"bar\";\
         \n  |              ^\
         \n  \'\
         \n  plain.css 1:14  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        fn nested() {
            let runner = runner().with_cwd("nested");
            assert_eq!(
                runner.ok("@import \'plain\'"),
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
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @include foo;\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn interpolation() {
        let runner = runner().with_cwd("interpolation");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @foo a#{b}c;\
         \n  |       ^^^^\
         \n  \'\
         \n  plain.css 1:7   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @mixin foo {\
         \n  | ^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn test_return() {
        let runner = runner().with_cwd("return");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @return foo;\
         \n  | ^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn warn() {
        let runner = runner().with_cwd("warn");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @warn foo;\
         \n  | ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_while() {
        let runner = runner().with_cwd("while");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @while false {\
         \n  | ^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
}
#[test]
#[ignore] // missing error
fn silent_comment() {
    let runner = runner().with_cwd("silent_comment");
    assert_eq!(
        runner.err("@import \'plain\'\n"),
        "Error: Silent comments aren\'t allowed in plain CSS.\
         \n  ,\
         \n1 | // silent\
         \n  | ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
mod style_rule {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("style_rule")
    }

    mod interpolation {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("interpolation")
        }

        #[test]
        #[ignore] // missing error
        fn custom_property() {
            let runner = runner().with_cwd("custom_property");
            assert_eq!(
                runner.err("@import \'plain\'"),
                "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   --b: #{c};\
         \n  |        ^^^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn declaration() {
            let runner = runner().with_cwd("declaration");
            assert_eq!(
                runner.err("@import \'plain\'"),
                "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w#{x}y: z;\
         \n  |    ^^^^\
         \n  \'\
         \n  plain.css 2:4   @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn selector() {
            let runner = runner().with_cwd("selector");
            assert_eq!(
                runner.err("@import \'plain\'"),
                "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a#{b}c {\
         \n  |  ^^^^\
         \n  \'\
         \n  plain.css 1:2   @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn nested() {
        let runner = runner().with_cwd("nested");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: expected \":\".\
         \n  ,\
         \n2 |   b {\
         \n  |     ^\
         \n  \'\
         \n  plain.css 2:5   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn nested_property() {
        let runner = runner().with_cwd("nested_property");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: Nested declarations aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: {\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn parent_selector() {
        let runner = runner().with_cwd("parent_selector");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &.foo {\
         \n  | ^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn placeholder_selector() {
        let runner = runner().with_cwd("placeholder_selector");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: Placeholder selectors aren\'t allowed here.\
         \n  ,\
         \n1 | %foo {\
         \n  | ^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
}
