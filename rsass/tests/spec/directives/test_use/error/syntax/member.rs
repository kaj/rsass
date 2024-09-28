//! Tests auto-converted from "sass-spec/spec/directives/use/error/syntax/member.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("member")
}

mod function {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn definition() {
        assert_eq!(
            runner().err("@function namespace.member() {@return null}\n"),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | @function namespace.member() {@return null}\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn no_member() {
        assert_eq!(
            runner().err("a {a: namespace.()}\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | a {a: namespace.()}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn no_namespace() {
        assert_eq!(
            runner().err("a {a: .member()}\n"),
            "Error: Expected digit.\
         \n  ,\
         \n1 | a {a: .member()}\
         \n  |        ^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn private() {
        assert_eq!(
        runner().err(
            "a {a: namespace._member()}\n"
        ),
        "Error: Private members can\'t be accessed from outside their modules.\
         \n  ,\
         \n1 | a {a: namespace._member()}\
         \n  |                 ^^^^^^^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
    );
    }
}
#[test]
#[ignore] // missing error
fn identifier_only() {
    assert_eq!(
        runner().err(
            "// A namespaced identifier with neither parentheses for a function nor a leading\
             \n// dollar sign for a variable is invalid syntax.\
             \na {a: namespace.member}\n"
        ),
        "Error: expected \"(\".\
         \n  ,\
         \n3 | a {a: namespace.member}\
         \n  |                       ^\
         \n  \'\
         \n  input.scss 3:23  root stylesheet",
    );
}
mod mixin {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn definition() {
        assert_eq!(
            runner().err("@mixin namespace.member() {}\n"),
            "Error: expected \"{\".\
         \n  ,\
         \n1 | @mixin namespace.member() {}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn no_member() {
        assert_eq!(
            runner().err("a {@include namespace.}\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | a {@include namespace.}\
         \n  |                       ^\
         \n  \'\
         \n  input.scss 1:23  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn no_namespace() {
        assert_eq!(
            runner().err("a {@include .member}\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | a {@include .member}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn private() {
        assert_eq!(
        runner().err(
            "a {@include namespace._member}\n"
        ),
        "Error: Private members can\'t be accessed from outside their modules.\
         \n  ,\
         \n1 | a {@include namespace._member}\
         \n  |                       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:23  root stylesheet",
    );
    }
}
#[test]
#[ignore] // missing error
fn unused_private() {
    assert_eq!(
        runner().err(
            "// Private member usage is a syntax error, so it should fail at parse time\
             \n// without needing to be executed.\
             \n@function foo() {\
             \n  @debug namespace.$_member;\
             \n}\n"
        ),
        "Error: Private members can\'t be accessed from outside their modules.\
         \n  ,\
         \n4 |   @debug namespace.$_member;\
         \n  |          ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:10  root stylesheet",
    );
}
mod variable {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn global() {
        assert_eq!(
            runner().err("namespace.$member: value !global;\n"),
            "Error: !global isn\'t allowed for variables in other modules.\
         \n  ,\
         \n1 | namespace.$member: value !global;\
         \n  |                          ^^^^^^^\
         \n  \'\
         \n  input.scss 1:26  root stylesheet",
        );
    }
    #[test]
    fn no_member() {
        assert_eq!(
            runner().err("a {a: namespace.$}\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | a {a: namespace.$}\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
        );
    }
    #[test]
    fn no_namespace() {
        assert_eq!(
            runner().err("a {a: $.member}\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | a {a: $.member}\
         \n  |        ^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn private() {
        assert_eq!(
        runner().err(
            "a {a: namespace.$_member}\n"
        ),
        "Error: Private members can\'t be accessed from outside their modules.\
         \n  ,\
         \n1 | a {a: namespace.$_member}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
