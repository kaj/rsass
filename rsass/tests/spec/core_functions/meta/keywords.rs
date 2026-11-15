//! Tests auto-converted from "sass-spec/spec/core_functions/meta/keywords.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("keywords")
        .mock_file("_utils.scss", "@use \"sass:meta\";\n/// Returns the keyword arguments passed to this function as a map.\n@function args-to-keywords($args...) {\n  @return meta.keywords($args);\n}\n")
}

#[test]
fn dash_insensitive() {
    let runner = runner().with_cwd("dash_insensitive");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords($c-d: e, $f_g: h))}\n"
        ),
        "a {\
         \n  b: (c-d: e, f-g: h);\
         \n}\n"
    );
}
mod empty {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("empty")
    }

    #[test]
    fn no_args() {
        let runner = runner().with_cwd("no_args");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"../../utils\";\
             \na {b: meta.inspect(utils.args-to-keywords())}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn positional() {
        let runner = runner().with_cwd("positional");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"../../utils\";\
             \na {b: meta.inspect(utils.args-to-keywords(1, 2, 3))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
}
mod error {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    fn too_few_args() {
        let runner = runner().with_cwd("too_few_args");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \na {b: meta.keywords()}\n"
            ),
            "Error: Missing argument $args.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.keywords()}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function keywords($args) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        let runner = runner().with_cwd("too_many_args");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \na {b: meta.keywords(1, 2)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.keywords(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function keywords($args) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("type")
        }

        #[test]
        #[ignore] // wrong error
        fn non_arg_list() {
            let runner = runner().with_cwd("non_arg_list");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \na {b: meta.keywords(1 2 3)}\n"
                ),
                "Error: $args: (1 2 3) is not an argument list.\
         \n  ,\
         \n2 | a {b: meta.keywords(1 2 3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn non_list() {
            let runner = runner().with_cwd("non_list");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \na {b: meta.keywords(1)}\n"
                ),
                "Error: $args: 1 is not an argument list.\
         \n  ,\
         \n2 | a {b: meta.keywords(1)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
mod escape {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("escape")
    }

    mod literal {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("literal")
        }

        #[test]
        #[ignore] // unexepected error
        fn non_normalized() {
            let runner = runner().with_cwd("non_normalized");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords($\\24: d))}\n"),
                "a {\
         \n  b: (\\$: d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn normalized() {
            let runner = runner().with_cwd("normalized");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords($\\$: d))}\n"),
                "a {\
         \n  b: (\\$: d);\
         \n}\n"
            );
        }
    }
    mod rest {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("rest")
        }

        mod quoted {
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("quoted")
            }

            #[test]
            #[ignore] // wrong result
            fn dash_insensitive() {
                let runner = runner().with_cwd("dash_insensitive");
                assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords((\"c\\\\5f d\": e)...))}\n"
        ),
        "a {\
         \n  b: (c\\5f d: e);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn non_normalized() {
                let runner = runner().with_cwd("non_normalized");
                assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords((\"\\\\24\": d)...))}\n"
        ),
        "a {\
         \n  b: (\\24: d);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn normalized() {
                let runner = runner().with_cwd("normalized");
                assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords((\"\\\\$\": d)...))}\n"
        ),
        "a {\
         \n  b: (\\$: d);\
         \n}\n"
    );
            }
            #[test]
            fn semantic() {
                let runner = runner().with_cwd("semantic");
                assert_eq!(
                    runner.ok("@use \"sass:meta\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords((\"$\": d)...))}\n"),
                    "a {\
         \n  b: ($: d);\
         \n}\n"
                );
            }
        }
        mod unquoted {
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("unquoted")
            }

            #[test]
            fn dash_insensitive() {
                let runner = runner().with_cwd("dash_insensitive");
                assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords((string.unquote(\"c\\\\5f d\"): e)...))}\n"
        ),
        "a {\
         \n  b: (c\\5f d: e);\
         \n}\n"
    );
            }
            #[test]
            fn non_normalized() {
                let runner = runner().with_cwd("non_normalized");
                assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords((string.unquote(\"\\\\24\"): d)...))}\n"
        ),
        "a {\
         \n  b: (\\24: d);\
         \n}\n"
    );
            }
            #[test]
            fn normalized() {
                let runner = runner().with_cwd("normalized");
                assert_eq!(
                    runner.ok("@use \"sass:meta\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords((\\$: d)...))}\n"),
                    "a {\
         \n  b: (\\$: d);\
         \n}\n"
                );
            }
            #[test]
            fn semantic() {
                let runner = runner().with_cwd("semantic");
                assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n@use \"core_functions/meta/keywords/utils\";\
             \na {b: meta.inspect(utils.args-to-keywords((string.unquote(\"$\"): d)...))}\n"
        ),
        "a {\
         \n  b: ($: d);\
         \n}\n"
    );
            }
        }
    }
}
mod forwarded {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("forwarded")
    }

    #[test]
    fn call() {
        let runner = runner().with_cwd("call");
        assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"../../utils\";\n\
             \n@function args-to-keywords-forward($args...) {\
             \n  @return meta.call(meta.get-function(\"args-to-keywords\", $module: \"utils\"), $args...);\
             \n}\n\
             \na {b: meta.inspect(args-to-keywords-forward($c: d))}\n"
        ),
        "a {\
         \n  b: (c: d);\
         \n}\n"
    );
    }
    #[test]
    fn content() {
        let runner = runner().with_cwd("content");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"../../utils\";\n\
             \n@mixin args-to-keywords-forward($args...) {\
             \n  @content($args...);\
             \n}\n\
             \n@include args-to-keywords-forward($c: d) using ($args...) {\
             \n  a {b: meta.inspect(utils.args-to-keywords($args...))}\
             \n}\n"),
            "a {\
         \n  b: (c: d);\
         \n}\n"
        );
    }
    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"../../utils\";\n\
             \n@function args-to-keywords-forward($args...) {\
             \n  @return utils.args-to-keywords($args...);\
             \n}\n\
             \na {b: meta.inspect(args-to-keywords-forward($c: d))}\n"),
            "a {\
         \n  b: (c: d);\
         \n}\n"
        );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"../../utils\";\n\
             \n@mixin args-to-keywords-forward($args...) {\
             \n  a {b: meta.inspect(utils.args-to-keywords($args...))}\
             \n}\n\
             \n@include args-to-keywords-forward($c: d);\n"),
            "a {\
         \n  b: (c: d);\
         \n}\n"
        );
    }
}
#[test]
fn multi_arg() {
    let runner = runner().with_cwd("multi_arg");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"../utils\";\
             \na {b: meta.inspect(utils.args-to-keywords($c: d, $e: f, $g: h))}\n"
        ),
        "a {\
         \n  b: (c: d, e: f, g: h);\
         \n}\n"
    );
}
#[test]
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@function args-to-keywords($args...) {\
             \n  @return meta.keywords($args: $args);\
             \n}\n\
             \na {b: meta.inspect(args-to-keywords($c: d))}\n"),
        "a {\
         \n  b: (c: d);\
         \n}\n"
    );
}
#[test]
fn one_arg() {
    let runner = runner().with_cwd("one_arg");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"../utils\";\
             \na {b: meta.inspect(utils.args-to-keywords($c: d))}\n"),
        "a {\
         \n  b: (c: d);\
         \n}\n"
    );
}
