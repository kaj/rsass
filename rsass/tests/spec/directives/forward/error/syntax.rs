//! Tests auto-converted from "sass-spec/spec/directives/forward/error/syntax.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("syntax")
}

mod after {
    use super::runner;

    mod at_rule {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn css() {
            assert_eq!(
        runner().err(
            "@keyframes a {};\
             \n@forward \"b\";\n"
        ),
        "Error: @forward rules must be written before any other rules.\
         \n  ,\
         \n2 | @forward \"b\";\
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
            "@import \"a\";\
             \n@forward \"b\";\n"
        ),
        "Error: @forward rules must be written before any other rules.\
         \n  ,\
         \n2 | @forward \"b\";\
         \n  | ^^^^^^^^^^^^\
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
             \n@forward \"a\";\n"
        ),
        "Error: @forward rules must be written before any other rules.\
         \n  ,\
         \n2 | @forward \"a\";\
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
            "@a;\
             \n@forward \"b\";\n"
        ),
        "Error: @forward rules must be written before any other rules.\
         \n  ,\
         \n2 | @forward \"b\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn style_rule() {
        assert_eq!(
            runner().err(
                "a {};\
             \n@forward \"b\";\n"
            ),
            "Error: @forward rules must be written before any other rules.\
         \n  ,\
         \n2 | @forward \"b\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
mod test_as {
    use super::runner;

    #[test]
    fn asterisk() {
        assert_eq!(
            runner().err("@forward \"a\" as *;\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | @forward \"a\" as *;\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
    #[test]
    fn invalid() {
        assert_eq!(
            runner().err("@forward \"a\" as 1-*;\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | @forward \"a\" as 1-*;\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
    #[test]
    fn no_star() {
        assert_eq!(
            runner().err("@forward \"a\" as foo;\n"),
            "Error: expected \"*\".\
         \n  ,\
         \n1 | @forward \"a\" as foo;\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
    #[test]
    fn nothing() {
        assert_eq!(
            runner().err("@forward \"a\" as;\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | @forward \"a\" as;\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
        );
    }
}
#[test]
fn empty() {
    assert_eq!(
        runner().err("@forward;\n"),
        "Error: Expected string.\
         \n  ,\
         \n1 | @forward;\
         \n  |         ^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
    );
}
mod hide {
    use super::runner;

    #[test]
    fn and_show() {
        assert_eq!(
            runner().err("@forward \"a\" hide b show c;\n"),
            "Error: expected \";\".\
         \n  ,\
         \n1 | @forward \"a\" hide b show c;\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
        );
    }
    #[test]
    fn empty_variable() {
        assert_eq!(
            runner().err("@forward \"a\" hide $;\n"),
            "Error: Expected variable, mixin, or function name\
         \n  ,\
         \n1 | @forward \"a\" hide $;\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
    #[test]
    fn invalid() {
        assert_eq!(
            runner().err("@forward \"a\" hide 1;\n"),
            "Error: Expected variable, mixin, or function name\
         \n  ,\
         \n1 | @forward \"a\" hide 1;\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
        );
    }
    #[test]
    fn nothing() {
        assert_eq!(
            runner().err("@forward \"a\" hide;\n"),
            "Error: Expected variable, mixin, or function name\
         \n  ,\
         \n1 | @forward \"a\" hide;\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
        );
    }
    #[test]
    fn trailing_comma() {
        assert_eq!(
            runner().err("@forward \"a\" hide b,;\n"),
            "Error: Expected variable, mixin, or function name\
         \n  ,\
         \n1 | @forward \"a\" hide b,;\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
        );
    }
}
mod show {
    use super::runner;

    #[test]
    fn and_hide() {
        assert_eq!(
            runner().err("@forward \"a\" show b hide c;\n"),
            "Error: expected \";\".\
         \n  ,\
         \n1 | @forward \"a\" show b hide c;\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
        );
    }
    #[test]
    fn empty_variable() {
        assert_eq!(
            runner().err("@forward \"a\" show $;\n"),
            "Error: Expected variable, mixin, or function name\
         \n  ,\
         \n1 | @forward \"a\" show $;\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
    #[test]
    fn invalid() {
        assert_eq!(
            runner().err("@forward \"a\" show 1;\n"),
            "Error: Expected variable, mixin, or function name\
         \n  ,\
         \n1 | @forward \"a\" show 1;\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
        );
    }
    #[test]
    fn nothing() {
        assert_eq!(
            runner().err("@forward \"a\" show;\n"),
            "Error: Expected variable, mixin, or function name\
         \n  ,\
         \n1 | @forward \"a\" show;\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
        );
    }
    #[test]
    fn trailing_comma() {
        assert_eq!(
            runner().err("@forward \"a\" show b,;\n"),
            "Error: Expected variable, mixin, or function name\
         \n  ,\
         \n1 | @forward \"a\" show b,;\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
        );
    }
}
mod url {
    use super::runner;

    #[test]
    fn unquoted() {
        assert_eq!(
            runner().err("@forward foo;\n"),
            "Error: Expected string.\
         \n  ,\
         \n1 | @forward foo;\
         \n  |          ^\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
        );
    }
}
mod with {
    use super::runner;

    #[test]
    fn before_as() {
        assert_eq!(
            runner().err("@forward \"other\" with ($a: b) as c-*;\n"),
            "Error: expected \";\".\
         \n  ,\
         \n1 | @forward \"other\" with ($a: b) as c-*;\
         \n  |                               ^\
         \n  \'\
         \n  input.scss 1:31  root stylesheet",
        );
    }
    #[test]
    fn before_hide() {
        assert_eq!(
            runner().err("@forward \"other\" with ($a: b) hide c;\n"),
            "Error: expected \";\".\
         \n  ,\
         \n1 | @forward \"other\" with ($a: b) hide c;\
         \n  |                               ^\
         \n  \'\
         \n  input.scss 1:31  root stylesheet",
        );
    }
    #[test]
    fn before_show() {
        assert_eq!(
            runner().err("@forward \"other\" with ($a: b) show c;\n"),
            "Error: expected \";\".\
         \n  ,\
         \n1 | @forward \"other\" with ($a: b) show c;\
         \n  |                               ^\
         \n  \'\
         \n  input.scss 1:31  root stylesheet",
        );
    }
    #[test]
    fn empty() {
        assert_eq!(
            runner().err("@forward \"other\" with ();\n"),
            "Error: expected \"$\".\
         \n  ,\
         \n1 | @forward \"other\" with ();\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
        );
    }
    #[test]
    fn extra_comma() {
        assert_eq!(
            runner().err("@forward \"other\" with ($a: b,,);\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @forward \"other\" with ($a: b,,);\
         \n  |                              ^\
         \n  \'\
         \n  input.scss 1:30  root stylesheet",
        );
    }
    #[test]
    fn missing_keyword() {
        assert_eq!(
            runner().err("@forward \"other\" with (a);\n"),
            "Error: expected \"$\".\
         \n  ,\
         \n1 | @forward \"other\" with (a);\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
        );
    }
    #[test]
    fn missing_value() {
        assert_eq!(
            runner().err("@forward \"other\" with ($a);\n"),
            "Error: expected \":\".\
         \n  ,\
         \n1 | @forward \"other\" with ($a);\
         \n  |                          ^\
         \n  \'\
         \n  input.scss 1:26  root stylesheet",
        );
    }
    #[test]
    fn namespace_variable() {
        assert_eq!(
            runner().err("@forward \"other\" with (a.$b: c);\n"),
            "Error: expected \"$\".\
         \n  ,\
         \n1 | @forward \"other\" with (a.$b: c);\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
        );
    }
    #[test]
    fn no_arguments() {
        assert_eq!(
            runner().err("@forward \"other\" with;\n"),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | @forward \"other\" with;\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
        );
    }
    #[test]
    fn space_after_dollar() {
        assert_eq!(
            runner().err("@forward \"other\" with ($ a: b);\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | @forward \"other\" with ($ a: b);\
         \n  |                         ^\
         \n  \'\
         \n  input.scss 1:25  root stylesheet",
        );
    }
}
mod within {
    use super::runner;

    #[test]
    fn function() {
        assert_eq!(
            runner().err(
                "@function a() {\
             \n  @forward \"b\";\
             \n}\n"
            ),
            "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @forward \"b\";\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
        );
    }
    #[test]
    fn mixin() {
        assert_eq!(
            runner().err(
                "@mixin a {\
             \n  @forward \"b\";\
             \n}\n"
            ),
            "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @forward \"b\";\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
        );
    }
    #[test]
    fn style_rule() {
        assert_eq!(
            runner().err(
                "a {\
             \n  @forward \"b\";\
             \n}\n"
            ),
            "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @forward \"b\";\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
        );
    }
}
