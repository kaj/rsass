//! Tests auto-converted from "sass-spec/spec/directives/use/error/syntax.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod after {
    #[allow(unused)]
    use super::runner;

    mod at_rule {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn css() {
            assert_eq!(
                runner().err(
                    "@keyframes foo {};\
             \n@use \"other\";\n"
                ),
                "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn import() {
            assert_eq!(
                runner().err(
                    "@import \"other1\";\
             \n@use \"other2\";\n"
                ),
                "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other2\";\
         \n  | ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn sass() {
            assert_eq!(
                runner().err(
                    "@if true {};\
             \n@use \"other\";\n"
                ),
                "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn unknown() {
            assert_eq!(
                runner().err(
                    "@fblthp;\
             \n@use \"other\";\n"
                ),
                "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
    }
    mod indented {
        #[allow(unused)]
        use super::runner;
    }
    #[test]
    #[ignore] // wrong error
    fn style_rule() {
        assert_eq!(
            runner().err(
                "a {};\
             \n@use \"other\";\n"
            ),
            "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn as_invalid() {
    assert_eq!(
        runner().err("@use \"foo\" as 1;\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | @use \"foo\" as 1;\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn as_nothing() {
    assert_eq!(
        runner().err("@use \"foo\" as;\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | @use \"foo\" as;\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn empty() {
    assert_eq!(
        runner().err("@use;\n"),
        "Error: Expected string.\
         \n  ,\
         \n1 | @use;\
         \n  |     ^\
         \n  \'\
         \n  input.scss 1:5  root stylesheet",
    );
}
mod member {
    #[allow(unused)]
    use super::runner;

    mod function {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
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
        #[ignore] // wrong error
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
        runner().err(
            "namespace.$member: value !global;\n"
        ),
        "Error: !global isn\'t allowed for variables in other modules.\
         \n  ,\
         \n1 | namespace.$member: value !global;\
         \n  |                          ^^^^^^^\
         \n  \'\
         \n  input.scss 1:26  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
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
        #[ignore] // wrong error
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
}
mod url {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn empty() {
        assert_eq!(
            runner().err("@use \"\";\n"),
            "Error: Invalid Sass identifier \"\"\
         \n  ,\
         \n1 | @use \"\";\
         \n  | ^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn non_identifier() {
        assert_eq!(
            runner().err("@use \"123\";\n"),
            "Error: Invalid Sass identifier \"123\"\
         \n  ,\
         \n1 | @use \"123\";\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn unquoted() {
        assert_eq!(
            runner().err("@use foo;\n"),
            "Error: Expected string.\
         \n  ,\
         \n1 | @use foo;\
         \n  |      ^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet",
        );
    }
}
mod with {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn before_as() {
        assert_eq!(
            runner().err("@use \"other\" with ($a: b) as c;\n\n"),
            "Error: expected \";\".\
         \n  ,\
         \n1 | @use \"other\" with ($a: b) as c;\
         \n  |                           ^\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn default() {
        assert_eq!(
            runner().err("@use \"other\" with ($a: b !default);\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @use \"other\" with ($a: b !default);\
         \n  |                          ^\
         \n  \'\
         \n  input.scss 1:26  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn empty() {
        assert_eq!(
            runner().err("@use \"other\" with ();\n"),
            "Error: expected \"$\".\
         \n  ,\
         \n1 | @use \"other\" with ();\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn extra_comma() {
        assert_eq!(
            runner().err("@use \"other\" with ($a: b,,);\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @use \"other\" with ($a: b,,);\
         \n  |                          ^\
         \n  \'\
         \n  input.scss 1:26  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn missing_keyword() {
        assert_eq!(
            runner().err("@use \"other\" with (a);\n"),
            "Error: expected \"$\".\
         \n  ,\
         \n1 | @use \"other\" with (a);\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn missing_value() {
        assert_eq!(
            runner().err("@use \"other\" with ($a);\n"),
            "Error: expected \":\".\
         \n  ,\
         \n1 | @use \"other\" with ($a);\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn namespace_variable() {
        assert_eq!(
            runner().err("@use \"other\" with (a.$b: c);\n"),
            "Error: expected \"$\".\
         \n  ,\
         \n1 | @use \"other\" with (a.$b: c);\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn no_arguments() {
        assert_eq!(
            runner().err("@use \"other\" with;\n"),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | @use \"other\" with;\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn space_after_dollar() {
        assert_eq!(
            runner().err("@use \"other\" with ($ a: b);\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | @use \"other\" with ($ a: b);\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
        );
    }
}
mod within {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn function() {
        assert_eq!(
            runner().err(
                "@function foo {\
             \n  @use \"other\";\
             \n}\n"
            ),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | @function foo {\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn mixin() {
        assert_eq!(
            runner().err(
                "@mixin foo {\
             \n  @use \"other\";\
             \n}\n"
            ),
            "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @use \"other\";\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn style_rule() {
        assert_eq!(
            runner().err(
                "a {\
             \n  @use \"other\";\
             \n}\n"
            ),
            "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @use \"other\";\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
        );
    }
}
