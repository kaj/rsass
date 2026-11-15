//! Tests auto-converted from "sass-spec/spec/callable/arguments/mixin.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("mixin")
        .mock_file("_utils.scss", "@use 'sass:meta';\n\n@mixin a($args...) {\n  b {\n    positional: meta.inspect($args);\n    named: meta.inspect(meta.keywords($args));\n  }\n}\n")
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
                "@use \'callable/arguments/mixin/utils\';\
             \n@include utils.a( , );\n"
            ),
            "Error: expected \")\".\
         \n  ,\
         \n2 | @include utils.a( , );\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 2:19  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn duplicate_named() {
        let runner = runner().with_cwd("duplicate_named");
        assert_eq!(
            runner.err(
                "@mixin a($b) {}\n\
             \n@include a($b: 1, $b: 2);\n"
            ),
            "Error: Duplicate argument.\
         \n  ,\
         \n3 | @include a($b: 1, $b: 2);\
         \n  |                   ^^\
         \n  \'\
         \n  input.scss 3:19  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn duplicate_named_normalization() {
        let runner = runner().with_cwd("duplicate_named_normalization");
        assert_eq!(
            runner.err(
                "@mixin a($b-c) {}\n\
             \n@include a($b-c: 1, $b_c: 2);\n"
            ),
            "Error: Duplicate argument.\
         \n  ,\
         \n3 | @include a($b-c: 1, $b_c: 2);\
         \n  |                     ^^^^\
         \n  \'\
         \n  input.scss 3:21  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn positional_after_named() {
        let runner = runner().with_cwd("positional_after_named");
        assert_eq!(
            runner.err(
                "@mixin a($b, $c) {}\n\
             \n@include a($b: 1, 2) {}\n"
            ),
            "Error: Positional arguments must come before keyword arguments.\
         \n  ,\
         \n3 | @include a($b: 1, 2) {}\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 3:19  root stylesheet",
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
             \n@mixin a($a, $b, $c) {}\
             \n@include a([1, 2]..., $c: 3);\n"
        ),
        ""
    );
        }
        #[test]
        fn before_positional() {
            let runner = runner().with_cwd("before_positional");
            assert_eq!(
        runner.ok(
            "// TODO - https://github.com/sass/sass-spec/issues/2073: This will be an error in Dart Sass 2\
             \n@mixin a($b, $args...) {}\
             \n@include a([1, 2]..., 3);\n"
        ),
        ""
    );
        }
    }
}
mod trailing_comma {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("trailing_comma")
    }

    #[test]
    fn keyword_rest() {
        let runner = runner().with_cwd("keyword_rest");
        assert_eq!(
            runner.ok("@use \'callable/arguments/mixin/utils\';\
             \n@include utils.a(1..., (c: 2)..., );\n"),
            "b {\
         \n  positional: (1,);\
         \n  named: (c: 2);\
         \n}\n"
        );
    }
    mod named {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("named")
        }

        #[test]
        fn after_positional() {
            let runner = runner().with_cwd("after_positional");
            assert_eq!(
                runner.ok("@use \'callable/arguments/mixin/utils\';\
             \n@include utils.a(1, $c: 2, );\n"),
                "b {\
         \n  positional: (1,);\
         \n  named: (c: 2);\
         \n}\n"
            );
        }
        #[test]
        fn alone() {
            let runner = runner().with_cwd("alone");
            assert_eq!(
                runner.ok("@use \'callable/arguments/mixin/utils\';\
             \n@include utils.a($c: 1, );\n"),
                "b {\
         \n  positional: ();\
         \n  named: (c: 1);\
         \n}\n"
            );
        }
    }
    #[test]
    fn positional() {
        let runner = runner().with_cwd("positional");
        assert_eq!(
            runner.ok("@use \'callable/arguments/mixin/utils\';\
             \n@include utils.a(1, );\n"),
            "b {\
         \n  positional: (1,);\
         \n  named: ();\
         \n}\n"
        );
    }
    mod rest {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("rest")
        }

        #[test]
        fn after_both() {
            let runner = runner().with_cwd("after_both");
            assert_eq!(
                runner.ok("@use \'callable/arguments/mixin/utils\';\
             \n@include utils.a(1, $c: 2, 3..., );\n"),
                "b {\
         \n  positional: 1, 3;\
         \n  named: (c: 2);\
         \n}\n"
            );
        }
        #[test]
        fn after_named() {
            let runner = runner().with_cwd("after_named");
            assert_eq!(
                runner.ok("@use \'callable/arguments/mixin/utils\';\
             \n@include utils.a($c: 1, 2..., );\n"),
                "b {\
         \n  positional: (2,);\
         \n  named: (c: 1);\
         \n}\n"
            );
        }
        #[test]
        fn after_positional() {
            let runner = runner().with_cwd("after_positional");
            assert_eq!(
                runner.ok("@use \'callable/arguments/mixin/utils\';\
             \n@include utils.a(1, 2..., );\n"),
                "b {\
         \n  positional: 1, 2;\
         \n  named: ();\
         \n}\n"
            );
        }
        #[test]
        fn alone() {
            let runner = runner().with_cwd("alone");
            assert_eq!(
                runner.ok("@use \'callable/arguments/mixin/utils\';\
             \n@include utils.a(1..., );\n"),
                "b {\
         \n  positional: (1,);\
         \n  named: ();\
         \n}\n"
            );
        }
    }
}
