//! Tests auto-converted from "sass-spec/spec/core_functions/meta/keywords.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("keywords")
        .mock_file("_utils.scss", "/// Returns the keyword arguments passed to this function as a map.\n@function args-to-keywords($args...) {\n  @return keywords($args);\n}\n")
}

#[test]
fn dash_insensitive() {
    let runner = runner().with_cwd("dash_insensitive");
    assert_eq!(
        runner.ok("@import \"../utils\";\
             \na {b: inspect(args-to-keywords($c-d: e, $f_g: h))}\n"),
        "a {\
         \n  b: (c-d: e, f-g: h);\
         \n}\n"
    );
}
mod empty {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("empty")
    }

    #[test]
    fn no_args() {
        let runner = runner().with_cwd("no_args");
        assert_eq!(
            runner.ok("@import \"../../utils\";\
             \na {b: inspect(args-to-keywords())}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn positional() {
        let runner = runner().with_cwd("positional");
        assert_eq!(
            runner.ok("@import \"../../utils\";\
             \na {b: inspect(args-to-keywords(1, 2, 3))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    fn too_few_args() {
        let runner = runner().with_cwd("too_few_args");
        assert_eq!(
            runner.err("a {b: keywords()}\n"),
            "Error: Missing argument $args.\
         \n  ,--> input.scss\
         \n1 | a {b: keywords()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function keywords($args) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        let runner = runner().with_cwd("too_many_args");
        assert_eq!(
            runner.err("a {b: keywords(1, 2)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: keywords(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function keywords($args) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("type")
        }

        #[test]
        fn non_arg_list() {
            let runner = runner().with_cwd("non_arg_list");
            assert_eq!(
                runner.err("a {b: keywords(1 2 3)}\n"),
                "Error: $args: 1 2 3 is not an argument list.\
         \n  ,\
         \n1 | a {b: keywords(1 2 3)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn non_list() {
            let runner = runner().with_cwd("non_list");
            assert_eq!(
                runner.err("a {b: keywords(1)}\n"),
                "Error: $args: 1 is not an argument list.\
         \n  ,\
         \n1 | a {b: keywords(1)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod forwarded {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("forwarded")
    }

    #[test]
    fn call() {
        let runner = runner().with_cwd("call");
        assert_eq!(
            runner.ok("@import \"../../utils\";\n\
             \n@function args-to-keywords-forward($args...) {\
             \n  @return call(get-function(\"args-to-keywords\"), $args...);\
             \n}\n\
             \na {b: inspect(args-to-keywords-forward($c: d))}\n"),
            "a {\
         \n  b: (c: d);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn content() {
        let runner = runner().with_cwd("content");
        assert_eq!(
            runner.ok("@import \"../../utils\";\n\
             \n@mixin args-to-keywords-forward($args...) {\
             \n  @content($args...);\
             \n}\n\
             \n@include args-to-keywords-forward($c: d) using ($args...) {\
             \n  a {b: inspect(args-to-keywords($args...))}\
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
            runner.ok("@import \"../../utils\";\n\
             \n@function args-to-keywords-forward($args...) {\
             \n  @return args-to-keywords($args...);\
             \n}\n\
             \na {b: inspect(args-to-keywords-forward($c: d))}\n"),
            "a {\
         \n  b: (c: d);\
         \n}\n"
        );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.ok("@import \"../../utils\";\n\
             \n@mixin args-to-keywords-forward($args...) {\
             \n  a {b: inspect(args-to-keywords($args...))}\
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
        runner.ok("@import \"../utils\";\
             \na {b: inspect(args-to-keywords($c: d, $e: f, $g: h))}\n"),
        "a {\
         \n  b: (c: d, e: f, g: h);\
         \n}\n"
    );
}
#[test]
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok("@function args-to-keywords($args...) {\
             \n  @return keywords($args: $args);\
             \n}\n\
             \na {b: inspect(args-to-keywords($c: d))}\n"),
        "a {\
         \n  b: (c: d);\
         \n}\n"
    );
}
#[test]
fn one_arg() {
    let runner = runner().with_cwd("one_arg");
    assert_eq!(
        runner.ok("@import \"../utils\";\
             \na {b: inspect(args-to-keywords($c: d))}\n"),
        "a {\
         \n  b: (c: d);\
         \n}\n"
    );
}
