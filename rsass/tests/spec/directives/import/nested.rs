//! Tests auto-converted from "sass-spec/spec/directives/import/nested.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("nested")
        .mock_file("at_rule/childless/_other.scss", "@b c;\n")
        .mock_file("at_rule/declaration_child/_other.scss", "@b {c: d}\n")
        .mock_file("at_rule/keyframes/_other.scss", "// This should ignore the parent selector, since Sass knows @keyframes is only\n// valid at the root of a document.\n@keyframes b {\n  0% {c: d}\n}\n")
        .mock_file("at_rule/rule_child/_other.scss", "@b {\n  c {d: e}\n}\n")
        .mock_file("scope/function/other.scss", "x {\n  function: local();\n}\n")
        .mock_file("scope/mixin/other.scss", "@include local;\n")
        .mock_file("scope/variable/other.scss", "x {\n  var: $var;\n}\n")
        .mock_file("with_comment/_a.scss", "/* Y */\n")
        .mock_file("with_comment/_b.scss", "@import 'a'\n")
        .mock_file("with_comment/_c.scss", "@import 'a'\n")
}

mod at_rule {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("at_rule")
    }

    #[test]
    fn childless() {
        let runner = runner().with_cwd("childless");
        assert_eq!(
            runner.ok("a {@import \"other\"}\n"),
            "a {\
         \n  @b c;\
         \n}\n"
        );
    }
    #[test]
    fn declaration_child() {
        let runner = runner().with_cwd("declaration_child");
        assert_eq!(
            runner.ok("a {@import \"other\"}\n"),
            "@b {\
         \n  a {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn keyframes() {
        let runner = runner().with_cwd("keyframes");
        assert_eq!(
            runner.ok("a {@import \"other\"}\n"),
            "@keyframes b {\
         \n  0% {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn rule_child() {
        let runner = runner().with_cwd("rule_child");
        assert_eq!(
            runner.ok("a {@import \"other\"}\n"),
            "@b {\
         \n  a c {\
         \n    d: e;\
         \n  }\
         \n}\n"
        );
    }
}
mod scope {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("scope")
    }

    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
        runner.ok(
            ".parent {\
             \n  // This should be visible to the imported stylesheet. There\'s not really a\
             \n  // good reason for this, but it\'s the historical behavior so whatever.\
             \n  @function local() {\
             \n    @return value;\
             \n  }\n\
             \n  @import \'other\';\
             \n}\n"
        ),
        ".parent x {\
         \n  function: value;\
         \n}\n"
    );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
        runner.ok(
            ".parent {\
             \n  // This should be visible to the imported stylesheet. There\'s not really a\
             \n  // good reason for this, but it\'s the historical behavior so whatever.\
             \n  @mixin local {\
             \n    x {y: z}\
             \n  }\n\
             \n  @import \'other\';\
             \n}\n"
        ),
        ".parent x {\
         \n  y: z;\
         \n}\n"
    );
    }
    #[test]
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
        runner.ok(
            ".parent {\
             \n  // This should be visible to the imported stylesheet. There\'s not really a\
             \n  // good reason for this, but it\'s the historical behavior so whatever.\
             \n  $var: value;\
             \n  @import \'other\';\
             \n}\n"
        ),
        ".parent x {\
         \n  var: value;\
         \n}\n"
    );
    }
}
#[test]
fn with_comment() {
    let runner = runner().with_cwd("with_comment");
    assert_eq!(
        runner.ok("@import \'b\';\
             \n@import \'c\';\n"),
        "/* Y */\
         \n/* Y */\n"
    );
}
