//! Tests auto-converted from "sass-spec/spec/css/functions/var.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("var")
        .mock_file("sass_function/_list-info.scss", "@use 'sass:list';\n@use 'sass:meta';\n@use 'sass:string';\n\n@function _is-quoted($string) {\n  @return meta.inspect($string) == meta.inspect(string.quote($string));\n}\n\n// A mixin that provides information about the structure of a list of strings.\n@mixin list-info($list) {\n  $quoted: [];\n  $is-quoted: [];\n  @each $element in $list {\n    $quoted: list.join($quoted, string.quote($element), $separator: comma);\n    $is-quoted: list.join($is-quoted, _is-quoted($element), $separator: comma);\n  }\n  quoted: $quoted;\n  is-quoted: $is-quoted;\n}\n")
}

mod css_function {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("css_function")
    }

    mod single_argument {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("single_argument")
        }

        #[test]
        fn expression() {
            let runner = runner().with_cwd("expression");
            assert_eq!(
                runner.ok("a {b: var(--c)}\n"),
                "a {\
         \n  b: var(--c);\
         \n}\n"
            );
        }
        #[test]
        fn rest() {
            let runner = runner().with_cwd("rest");
            assert_eq!(
                runner.ok("$name: --c;\
             \na {b: var($name...)}\n"),
                "a {\
         \n  b: var(--c);\
         \n}\n"
            );
        }
    }
    #[test]
    fn three_argument() {
        let runner = runner().with_cwd("three_argument");
        assert_eq!(
            runner.ok("a {b: var(--c, d, e)}\n"),
            "a {\
         \n  b: var(--c, d, e);\
         \n}\n"
        );
    }
    mod two_argument {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("two_argument")
        }

        #[test]
        fn dynamic() {
            let runner = runner().with_cwd("dynamic");
            assert_eq!(
                runner.ok("a {b: var(--c, 1 + 2)}\n"),
                "a {\
         \n  b: var(--c, 3);\
         \n}\n"
            );
        }
        mod empty {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("empty")
            }

            #[test]
            fn case_insensitive() {
                let runner = runner().with_cwd("case_insensitive");
                assert_eq!(
                    runner.ok("a {b: VaR(--c,)}\n"),
                    "a {\
         \n  b: VaR(--c, );\
         \n}\n"
                );
            }
            #[test]
            fn no_whitespace() {
                let runner = runner().with_cwd("no_whitespace");
                assert_eq!(
                    runner.ok("a {b: var(--c,)}\n"),
                    "a {\
         \n  b: var(--c, );\
         \n}\n"
                );
            }
            #[test]
            fn whitespace_after() {
                let runner = runner().with_cwd("whitespace_after");
                assert_eq!(
                    runner.ok("a {b: var(--c, )}\n"),
                    "a {\
         \n  b: var(--c, );\
         \n}\n"
                );
            }
            #[test]
            fn whitespace_around() {
                let runner = runner().with_cwd("whitespace_around");
                assert_eq!(
                    runner.ok("a {b: var(--c , )}\n"),
                    "a {\
         \n  b: var(--c, );\
         \n}\n"
                );
            }
            #[test]
            fn whitespace_before() {
                let runner = runner().with_cwd("whitespace_before");
                assert_eq!(
                    runner.ok("a {b: var(--c ,)}\n"),
                    "a {\
         \n  b: var(--c, );\
         \n}\n"
                );
            }
        }
        #[test]
        fn expressions() {
            let runner = runner().with_cwd("expressions");
            assert_eq!(
                runner.ok("a {b: var(--c, d)}\n"),
                "a {\
         \n  b: var(--c, d);\
         \n}\n"
            );
        }
        #[test]
        fn rest() {
            let runner = runner().with_cwd("rest");
            assert_eq!(
                runner.ok("$args: --c, d;\
             \na {b: var($args...)}\n"),
                "a {\
         \n  b: var(--c, d);\
         \n}\n"
            );
        }
    }
    mod zero_argument {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("zero_argument")
        }

        #[test]
        fn empty() {
            let runner = runner().with_cwd("empty");
            assert_eq!(
                runner.ok("a {b: var()}\n"),
                "a {\
         \n  b: var();\
         \n}\n"
            );
        }
    }
}
mod error {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    fn empty_after_keyword() {
        let runner = runner().with_cwd("empty_after_keyword");
        assert_eq!(
            runner.err(
                "@function var($name, $arg) {@return null}\
             \na {b: var($name: --c, )}\n"
            ),
            "Error: Missing argument $arg.\
         \n  ,\
         \n1 | @function var($name, $arg) {@return null}\
         \n  |           ================ declaration\
         \n2 | a {b: var($name: --c, )}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  input.scss 2:7  var()\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn empty_second_before_third() {
        let runner = runner().with_cwd("empty_second_before_third");
        assert_eq!(
            runner.err("a {b: var(--c, , d)}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | a {b: var(--c, , d)}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn invalid_second_arg_syntax() {
        let runner = runner().with_cwd("invalid_second_arg_syntax");
        assert_eq!(
        runner.err(
            "// The second argument is *not* parsed as a `<declaration-value>`.\
             \na {b: var(--c, {})}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n2 | a {b: var(--c, {})}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 2:16  root stylesheet",
    );
    }
}
mod sass_function {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("sass_function")
    }

    mod normal_trailing_comma_behavior {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("normal_trailing_comma_behavior")
        }

        #[test]
        fn empty_after_named() {
            let runner = runner().with_cwd("empty_after_named");
            assert_eq!(
        runner.ok(
            "@use \'css/functions/var/sass_function/list-info\' as *;\
             \n@function var($arg) {@return [$arg]}\
             \na {@include list-info(var($arg: --c, ))}\n"
        ),
        "a {\
         \n  quoted: [\"--c\"];\
         \n  is-quoted: [false];\
         \n}\n"
    );
        }
        #[test]
        fn empty_after_rest() {
            let runner = runner().with_cwd("empty_after_rest");
            assert_eq!(
        runner.ok(
            "@use \'css/functions/var/sass_function/list-info\' as *;\
             \n@function var($args...) {@return $args}\n\
             \n$name: --c;\
             \na {@include list-info(var($name..., ))}\n"
        ),
        "a {\
         \n  quoted: [\"--c\"];\
         \n  is-quoted: [false];\
         \n}\n"
    );
        }
    }
    mod single_argument {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("single_argument")
        }

        #[test]
        fn expression() {
            let runner = runner().with_cwd("expression");
            assert_eq!(
        runner.ok(
            "@use \'css/functions/var/sass_function/list-info\' as *;\
             \n@function var($args...) {@return $args}\
             \na {@include list-info(var(--c))}\n"
        ),
        "a {\
         \n  quoted: [\"--c\"];\
         \n  is-quoted: [false];\
         \n}\n"
    );
        }
        #[test]
        fn rest() {
            let runner = runner().with_cwd("rest");
            assert_eq!(
        runner.ok(
            "@use \'css/functions/var/sass_function/list-info\' as *;\
             \n@function var($args...) {@return $args}\
             \n$name: --c;\
             \na {@include list-info(var($name...))}\n"
        ),
        "a {\
         \n  quoted: [\"--c\"];\
         \n  is-quoted: [false];\
         \n}\n"
    );
        }
    }
    #[test]
    fn three_argument() {
        let runner = runner().with_cwd("three_argument");
        assert_eq!(
            runner.ok(
                "@use \'css/functions/var/sass_function/list-info\' as *;\
             \n@function var($args...) {@return $args}\
             \na {@include list-info(var(--c, d, e))}\n"
            ),
            "a {\
         \n  quoted: [\"--c\", \"d\", \"e\"];\
         \n  is-quoted: [false, false, false];\
         \n}\n"
        );
    }
    mod two_argument {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("two_argument")
        }

        #[test]
        fn doesnt_use_function_if_case_doesnt_match() {
            let runner =
                runner().with_cwd("doesnt_use_function_if_case_doesnt_match");
            assert_eq!(
                runner.ok("@function var($args...) {@return $args}\
             \na {b: VaR(--c, )}\n"),
                "a {\
         \n  b: VaR(--c, );\
         \n}\n"
            );
        }
        #[test]
        fn dynamic() {
            let runner = runner().with_cwd("dynamic");
            assert_eq!(
        runner.ok(
            "@use \'css/functions/var/sass_function/list-info\' as *;\
             \n@function var($args...) {@return $args}\
             \na {@include list-info(var(--c, \"d\" + \"e\"))}\n"
        ),
        "a {\
         \n  quoted: [\"--c\", \"de\"];\
         \n  is-quoted: [false, true];\
         \n}\n"
    );
        }
        #[test]
        fn empty() {
            let runner = runner().with_cwd("empty");
            assert_eq!(
        runner.ok(
            "@use \'css/functions/var/sass_function/list-info\' as *;\
             \n@function var($args...) {@return $args}\
             \na {@include list-info(var(--c, ))}\n"
        ),
        "a {\
         \n  quoted: [\"--c\", \"\"];\
         \n  is-quoted: [false, false];\
         \n}\n"
    );
        }
        #[test]
        fn expressions() {
            let runner = runner().with_cwd("expressions");
            assert_eq!(
        runner.ok(
            "@use \'css/functions/var/sass_function/list-info\' as *;\
             \n@function var($args...) {@return $args}\
             \na {@include list-info(var(--c, d))}\n"
        ),
        "a {\
         \n  quoted: [\"--c\", \"d\"];\
         \n  is-quoted: [false, false];\
         \n}\n"
    );
        }
        #[test]
        fn rest() {
            let runner = runner().with_cwd("rest");
            assert_eq!(
        runner.ok(
            "@use \'css/functions/var/sass_function/list-info\' as *;\
             \n@function var($args...) {@return $args}\
             \n$args: --c, d;\
             \na {@include list-info(var($args...))}\n"
        ),
        "a {\
         \n  quoted: [\"--c\", \"d\"];\
         \n  is-quoted: [false, false];\
         \n}\n"
    );
        }
    }
    #[test]
    fn zero_argument() {
        let runner = runner().with_cwd("zero_argument");
        assert_eq!(
            runner.ok(
                "@use \'css/functions/var/sass_function/list-info\' as *;\
             \n@function var($args...) {@return $args}\
             \na {@include list-info(var())}\n"
            ),
            "a {\
         \n  quoted: [];\
         \n  is-quoted: [];\
         \n}\n"
        );
    }
}
