//! Tests auto-converted from "sass-spec/spec/css/plain/error/statement/style_rule.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("style_rule")
        .mock_file(
            "interpolation/custom_property/plain.css",
            "a {\n  --b: #{c};\n}\n",
        )
        .mock_file(
            "interpolation/declaration/plain.css",
            "a {\n  w#{x}y: z;\n}\n",
        )
        .mock_file(
            "interpolation/selector/plain.css",
            "a#{b}c {\n  x: y;\n}\n",
        )
        .mock_file(
            "leading_combinator/through_import/plain.css",
            "> b {c: d}\n",
        )
        .mock_file(
            "leading_combinator/through_load_css/plain.css",
            "> b {c: d}\n",
        )
        .mock_file("leading_combinator/top_level/plain.css", "> a {b: c}\n")
        .mock_file(
            "nested_property/no_value/plain.css",
            "a {\n  x: {\n    y: z;\n  }\n}\n",
        )
        .mock_file(
            "nested_property/value/plain.css",
            "a {\n  b: c {\n    d: e;\n  }\n}\n",
        )
        .mock_file("parent_selector/suffix/plain.css", "a {&b {c: d}}\n")
        .mock_file("placeholder_selector/plain.css", "%foo {\n  x: y;\n}\n")
        .mock_file(
            "trailing_combinator/nesting/plain.css",
            "a > {b {c: d}}\n",
        )
        .mock_file("trailing_combinator/no_nesting/plain.css", "a > {b: c}\n")
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
            runner.err("@use \'plain\'"),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   --b: #{c};\
         \n  |        ^^^^\
         \n  \'\
         \n  plain.css 2:8   @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn declaration() {
        let runner = runner().with_cwd("declaration");
        assert_eq!(
            runner.err("@use \'plain\'"),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w#{x}y: z;\
         \n  |    ^^^^\
         \n  \'\
         \n  plain.css 2:4   @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn selector() {
        let runner = runner().with_cwd("selector");
        assert_eq!(
            runner.err("@use \'plain\'"),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a#{b}c {\
         \n  |  ^^^^\
         \n  \'\
         \n  plain.css 1:2   @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
}
mod leading_combinator {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("leading_combinator")
    }

    #[test]
    #[ignore] // missing error
    fn through_import() {
        let runner = runner().with_cwd("through_import");
        assert_eq!(
        runner.err(
            "a {@import \"plain\"}\n"
        ),
        "Error: Top-level leading combinators aren\'t allowed in plain CSS.\
         \n  ,\
         \n1 | > b {c: d}\
         \n  | ^\
         \n  \'\
         \n  plain.css 1:1    @import\
         \n  input.scss 1:12  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn through_load_css() {
        let runner = runner().with_cwd("through_load_css");
        assert_eq!(
        runner.err(
            "@use \"sass:meta\";\n\
             \na {@include meta.load-css(\"plain\")}\n"
        ),
        "Error: Top-level leading combinators aren\'t allowed in plain CSS.\
         \n  ,\
         \n1 | > b {c: d}\
         \n  | ^\
         \n  \'\
         \n  plain.css 1:1   load-css()\
         \n  input.scss 3:4  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn top_level() {
        let runner = runner().with_cwd("top_level");
        assert_eq!(
        runner.err(
            "@use \'plain\';\n"
        ),
        "Error: Top-level leading combinators aren\'t allowed in plain CSS.\
         \n  ,\
         \n1 | > a {b: c}\
         \n  | ^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
    }
}
mod nested_property {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("nested_property")
    }

    #[test]
    #[ignore] // wrong error
    fn no_value() {
        let runner = runner().with_cwd("no_value");
        assert_eq!(
            runner.err("@use \'plain\'"),
            "Error: Nested declarations aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: {\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn value() {
        let runner = runner().with_cwd("value");
        assert_eq!(
            runner.err("@use \'plain\'\n"),
            "Error: Nested declarations aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   b: c {\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
}
mod parent_selector {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("parent_selector")
    }

    #[test]
    #[ignore] // wrong error
    fn suffix() {
        let runner = runner().with_cwd("suffix");
        assert_eq!(
            runner.err("@use \'plain\';\n"),
            "Error: Parent selectors can\'t have suffixes in plain CSS.\
         \n  ,\
         \n1 | a {&b {c: d}}\
         \n  |    ^^\
         \n  \'\
         \n  plain.css 1:4   @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
}
#[test]
#[ignore] // missing error
fn placeholder_selector() {
    let runner = runner().with_cwd("placeholder_selector");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: Placeholder selectors aren\'t allowed in plain CSS.\
         \n  ,\
         \n1 | %foo {\
         \n  | ^^^^\
         \n  \'\
         \n  plain.css 1:1   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
mod trailing_combinator {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("trailing_combinator")
    }

    #[test]
    #[ignore] // wrong error
    fn nesting() {
        let runner = runner().with_cwd("nesting");
        assert_eq!(
            runner.err("@use \'plain\';\n"),
            "Error: expected selector.\
         \n  ,\
         \n1 | a > {b {c: d}}\
         \n  |     ^\
         \n  \'\
         \n  plain.css 1:5   @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn no_nesting() {
        let runner = runner().with_cwd("no_nesting");
        assert_eq!(
            runner.err("@use \'plain\';\n"),
            "Error: expected selector.\
         \n  ,\
         \n1 | a > {b: c}\
         \n  |     ^\
         \n  \'\
         \n  plain.css 1:5   @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
}
