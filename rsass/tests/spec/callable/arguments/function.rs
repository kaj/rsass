//! Tests auto-converted from "sass-spec/spec/callable/arguments/function.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("function")
        .mock_file("_utils.scss", "@use 'sass:meta';\n\n@function a($args...) {\n  @return meta.inspect((\n    positional: $args,\n    named: meta.keywords($args),\n  ));\n}\n")
}

mod error {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    fn comma_only() {
        let runner = runner().with_cwd("comma_only");
        assert_eq!(
            runner.err(
                "@use \'callable/arguments/function/utils\';\
             \na {b: utils.a( , )};\n"
            ),
            "Error: expected \")\".\
         \n  ,\
         \n2 | a {b: utils.a( , )};\
         \n  |                ^\
         \n  \'\
         \n  input.scss 2:16  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn positional_after_named() {
        let runner = runner().with_cwd("positional_after_named");
        assert_eq!(
            runner.err(
                "@function a($b, $c) {@return null}\n\
             \n$d: a($b: 1, 2);\n"
            ),
            "Error: Positional arguments must come before keyword arguments.\
         \n  ,\
         \n3 | $d: a($b: 1, 2);\
         \n  |              ^\
         \n  \'\
         \n  input.scss 3:14  root stylesheet",
        );
    }
    mod splat {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("splat")
        }

        #[test]
        fn before_named() {
            let runner = runner().with_cwd("before_named");
            assert_eq!(
        runner.ok(
            "// TODO - https://github.com/sass/sass-spec/issues/2073: This will be an error in Dart Sass 2\
             \na {b: rgb([1, 2]..., $blue: 3)}\n"
        ),
        "a {\
         \n  b: rgb(1, 2, 3);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn before_positional() {
            let runner = runner().with_cwd("before_positional");
            assert_eq!(
        runner.ok(
            "// TODO - https://github.com/sass/sass-spec/issues/2073: This will be an error in Dart Sass 2\
             \na {b: rgb([1, 2]..., 3)}\n"
        ),
        "a {\
         \n  b: rgb(3, 1, 2);\
         \n}\n"
    );
        }
    }
}
mod trailing_comma {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("trailing_comma")
    }

    #[test]
    #[ignore] // wrong result
    fn keyword_rest() {
        let runner = runner().with_cwd("keyword_rest");
        assert_eq!(
            runner.ok("@use \'callable/arguments/function/utils\';\
             \na {b: utils.a(1..., (c: 2)..., )}\n"),
            "a {\
         \n  b: (positional: ((1,)), named: (c: 2));\
         \n}\n"
        );
    }
    mod named {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("named")
        }

        #[test]
        #[ignore] // wrong result
        fn after_positional() {
            let runner = runner().with_cwd("after_positional");
            assert_eq!(
                runner.ok("@use \'callable/arguments/function/utils\';\
             \na {b: utils.a(1, $c: 2, )}\n"),
                "a {\
         \n  b: (positional: ((1,)), named: (c: 2));\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn alone() {
            let runner = runner().with_cwd("alone");
            assert_eq!(
                runner.ok("@use \'callable/arguments/function/utils\';\
             \na {b: utils.a($c: 1, )}\n"),
                "a {\
         \n  b: (positional: (()), named: (c: 1));\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn positional() {
        let runner = runner().with_cwd("positional");
        assert_eq!(
            runner.ok("@use \'callable/arguments/function/utils\';\
             \na {b: utils.a(1, )}\n"),
            "a {\
         \n  b: (positional: ((1,)), named: ());\
         \n}\n"
        );
    }
    mod rest {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("rest")
        }

        #[test]
        #[ignore] // wrong result
        fn after_both() {
            let runner = runner().with_cwd("after_both");
            assert_eq!(
                runner.ok("@use \'callable/arguments/function/utils\';\
             \na {b: utils.a(1, $c: 2, 3..., )}\n"),
                "a {\
         \n  b: (positional: (1, 3), named: (c: 2));\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn after_named() {
            let runner = runner().with_cwd("after_named");
            assert_eq!(
                runner.ok("@use \'callable/arguments/function/utils\';\
             \na {b: utils.a($c: 1, 2..., )}\n"),
                "a {\
         \n  b: (positional: ((2,)), named: (c: 1));\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn after_positional() {
            let runner = runner().with_cwd("after_positional");
            assert_eq!(
                runner.ok("@use \'callable/arguments/function/utils\';\
             \na {b: utils.a(1, 2..., )}\n"),
                "a {\
         \n  b: (positional: (1, 2), named: ());\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn alone() {
            let runner = runner().with_cwd("alone");
            assert_eq!(
                runner.ok("@use \'callable/arguments/function/utils\';\
             \na {b: utils.a(1..., )}\n"),
                "a {\
         \n  b: (positional: ((1,)), named: ());\
         \n}\n"
            );
        }
    }
}
