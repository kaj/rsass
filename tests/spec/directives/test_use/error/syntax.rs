//! Tests auto-converted from "sass-spec/spec/directives/use/error/syntax.hrx"

mod after {
    mod at_rule {
        #[test]
        #[ignore] // wrong error
        fn css() {
            assert_eq!(
                crate::rsass(
                    "@keyframes foo {};\
             \n@use \"other\";\
             \n"
                )
                .unwrap_err(),
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
                crate::rsass(
                    "@import \"other1\";\
             \n@use \"other2\";\
             \n"
                )
                .unwrap_err(),
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
                crate::rsass(
                    "@if true {};\
             \n@use \"other\";\
             \n"
                )
                .unwrap_err(),
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
                crate::rsass(
                    "@fblthp;\
             \n@use \"other\";\
             \n"
                )
                .unwrap_err(),
                "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
    }
    mod indented {}
    #[test]
    #[ignore] // wrong error
    fn style_rule() {
        assert_eq!(
            crate::rsass(
                "a {};\
             \n@use \"other\";\
             \n"
            )
            .unwrap_err(),
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
        crate::rsass(
            "@use \"foo\" as 1;\
             \n"
        )
        .unwrap_err(),
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
        crate::rsass(
            "@use \"foo\" as;\
             \n"
        )
        .unwrap_err(),
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
        crate::rsass(
            "@use;\
             \n"
        )
        .unwrap_err(),
        "Error: Expected string.\
         \n  ,\
         \n1 | @use;\
         \n  |     ^\
         \n  \'\
         \n  input.scss 1:5  root stylesheet",
    );
}
mod member {
    mod function {
        #[test]
        #[ignore] // wrong error
        fn definition() {
            assert_eq!(
                crate::rsass(
                    "@function namespace.member() {@return null}\
             \n"
                )
                .unwrap_err(),
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
                crate::rsass(
                    "a {a: namespace.()}\
             \n"
                )
                .unwrap_err(),
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
                crate::rsass(
                    "a {a: .member()}\
             \n"
                )
                .unwrap_err(),
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
        crate::rsass(
            "a {a: namespace._member()}\
             \n"
        ).unwrap_err(),
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
        crate::rsass(
            "// A namespaced identifier with neither parentheses for a function nor a leading\
             \n// dollar sign for a variable is invalid syntax.\
             \na {a: namespace.member}\
             \n"
        ).unwrap_err(),
        "Error: expected \"(\".\
         \n  ,\
         \n3 | a {a: namespace.member}\
         \n  |                       ^\
         \n  \'\
         \n  input.scss 3:23  root stylesheet",
    );
    }
    mod mixin {
        #[test]
        #[ignore] // wrong error
        fn definition() {
            assert_eq!(
                crate::rsass(
                    "@mixin namespace.member() {}\
             \n"
                )
                .unwrap_err(),
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
                crate::rsass(
                    "a {@include namespace.}\
             \n"
                )
                .unwrap_err(),
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
                crate::rsass(
                    "a {@include .member}\
             \n"
                )
                .unwrap_err(),
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
        crate::rsass(
            "a {@include namespace._member}\
             \n"
        ).unwrap_err(),
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
        crate::rsass(
            "// Private member usage is a syntax error, so it should fail at parse time\
             \n// without needing to be executed.\
             \n@function foo() {\
             \n  @debug namespace.$_member;\
             \n}\
             \n"
        ).unwrap_err(),
        "Error: Private members can\'t be accessed from outside their modules.\
         \n  ,\
         \n4 |   @debug namespace.$_member;\
         \n  |          ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:10  root stylesheet",
    );
    }
    mod variable {
        #[test]
        #[ignore] // wrong error
        fn global() {
            assert_eq!(
        crate::rsass(
            "namespace.$member: value !global;\
             \n"
        ).unwrap_err(),
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
                crate::rsass(
                    "a {a: namespace.$}\
             \n"
                )
                .unwrap_err(),
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
                crate::rsass(
                    "a {a: $.member}\
             \n"
                )
                .unwrap_err(),
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
        crate::rsass(
            "a {a: namespace.$_member}\
             \n"
        ).unwrap_err(),
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
    #[test]
    #[ignore] // wrong error
    fn empty() {
        assert_eq!(
            crate::rsass(
                "@use \"\";\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \"123\";\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use foo;\
             \n"
            )
            .unwrap_err(),
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
    #[test]
    #[ignore] // wrong error
    fn before_as() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" with ($a: b) as c;\
             \n\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \"other\" with ($a: b !default);\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \"other\" with ();\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \"other\" with ($a: b,,);\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \"other\" with (a);\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \"other\" with ($a);\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \"other\" with (a.$b: c);\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \"other\" with;\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \"other\" with ($ a: b);\
             \n"
            )
            .unwrap_err(),
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
    #[test]
    #[ignore] // wrong error
    fn function() {
        assert_eq!(
            crate::rsass(
                "@function foo {\
             \n  @use \"other\";\
             \n}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@mixin foo {\
             \n  @use \"other\";\
             \n}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "a {\
             \n  @use \"other\";\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @use \"other\";\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
        );
    }
}
