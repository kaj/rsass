//! Tests auto-converted from "sass-spec/spec/directives/forward/error/member/inaccessible.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("inaccessible")
        .mock_file(
            "hidden/as/different_separator/_midstream.scss",
            "@forward \"upstream\" as b-* hide b_a;\n",
        )
        .mock_file(
            "hidden/as/different_separator/_upstream.scss",
            "@as a {b {c: d}}\n",
        )
        .mock_file(
            "hidden/as/same_separator/_midstream.scss",
            "@forward \"upstream\" as b-* hide b-a;\n",
        )
        .mock_file(
            "hidden/as/same_separator/_upstream.scss",
            "@as a {b {c: d}}\n",
        )
        .mock_file(
            "hidden/mixin/_midstream.scss",
            "@forward \"upstream\" hide a;\n",
        )
        .mock_file("hidden/mixin/_upstream.scss", "@mixin a {b {c: d}}\n")
        .mock_file(
            "hidden/variable/_midstream.scss",
            "@forward \"upstream\" hide $c;\n",
        )
        .mock_file("hidden/variable/_upstream.scss", "$c: d;\n")
        .mock_file(
            "local/function/_other.scss",
            "@function c() {@return d}\n",
        )
        .mock_file("local/mixin/_other.scss", "@mixin a {b {c: d}}\n")
        .mock_file("local/variable/_other.scss", "$c: d;\n")
        .mock_file(
            "not_shown/as/mixin/_midstream.scss",
            "@forward \"upstream\" as b-* show a;\n",
        )
        .mock_file(
            "not_shown/as/mixin/_upstream.scss",
            "@mixin a {c {d: e}}\n",
        )
        .mock_file(
            "not_shown/mixin/_midstream.scss",
            "@forward \"upstream\" show b;\n",
        )
        .mock_file("not_shown/mixin/_upstream.scss", "@mixin a {c {d: e}}\n")
        .mock_file(
            "not_shown/variable/_midstream.scss",
            "@forward \"upstream\" show $d;\n",
        )
        .mock_file("not_shown/variable/_upstream.scss", "$c: e;\n")
        .mock_file(
            "not_shown/wrong_type/mixin/_midstream.scss",
            "@forward \"upstream\" show $a;\n",
        )
        .mock_file(
            "not_shown/wrong_type/mixin/_upstream.scss",
            "@mixin a {c {d: e}}\n",
        )
        .mock_file(
            "not_shown/wrong_type/variable/_midstream.scss",
            "@forward \"upstream\" show c;\n",
        )
        .mock_file("not_shown/wrong_type/variable/_upstream.scss", "$c: e;\n")
        .mock_file(
            "private/function/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "private/function/_upstream.scss",
            "@function -c() {@return d}\n",
        )
        .mock_file(
            "private/mixin/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file("private/mixin/_upstream.scss", "@mixin -a {b {c: d}}\n")
        .mock_file(
            "private/variable/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file("private/variable/_upstream.scss", "$-c: d;\n")
}

mod hidden {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("hidden")
    }

    mod test_as {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("as")
        }

        #[test]
        fn different_separator() {
            let runner = runner().with_cwd("different_separator");
            assert_eq!(
                runner.err(
                    "@use \"midstream\" as *;\n\
             \n@include a;\n"
                ),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include a;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        fn same_separator() {
            let runner = runner().with_cwd("same_separator");
            assert_eq!(
                runner.err(
                    "@use \"midstream\" as *;\n\
             \n@include a;\n"
                ),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include a;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.err(
                "@use \"midstream\" as *;\n\
             \n@include a;\n"
            ),
            "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include a;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
        );
    }
    #[test]
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.err(
                "@use \"midstream\" as *;\n\
             \na {b: $c}\n"
            ),
            "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $c}\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
}
mod local {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("local")
    }

    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
        runner.ok(
            "@forward \"other\";\n\
             \n// This is technically not a compile error, since `-member()` is treated as\
             \n// plain CSS, but it\'s included here for consistency with the other specs.\
             \na {b: c()};\n"
        ),
        "a {\
         \n  b: c();\
         \n}\n"
    );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.err(
                "@forward \"other\";\n\
             \n@include a;\n"
            ),
            "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include a;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
        );
    }
    #[test]
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.err(
                "@forward \"other\";\n\
             \na {b: $c};\n"
            ),
            "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $c};\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
}
mod not_shown {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("not_shown")
    }

    mod test_as {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("as")
        }

        #[test]
        fn mixin() {
            let runner = runner().with_cwd("mixin");
            assert_eq!(
                runner.err(
                    "@use \"midstream\" as *;\n\
             \n@include b-a;\n"
                ),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include b-a;\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.err(
                "@use \"midstream\" as *;\n\
             \n@include a;\n"
            ),
            "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include a;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
        );
    }
    #[test]
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.err(
                "@use \"midstream\" as *;\n\
             \na {b: $c}\n"
            ),
            "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $c}\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
    mod wrong_type {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("wrong_type")
        }

        #[test]
        fn mixin() {
            let runner = runner().with_cwd("mixin");
            assert_eq!(
                runner.err(
                    "@use \"midstream\" as *;\n\
             \n@include a;\n"
                ),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include a;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        fn variable() {
            let runner = runner().with_cwd("variable");
            assert_eq!(
                runner.err(
                    "@use \"midstream\" as *;\n\
             \na {b: $c}\n"
                ),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $c}\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
}
mod private {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("private")
    }

    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
        runner.ok(
            "@use \"midstream\" as *;\n\
             \n// This is technically not a compile error, since `-member()` is treated as\
             \n// plain CSS, but it\'s included here for consistency with the other specs.\
             \na {b: -c()};\n"
        ),
        "a {\
         \n  b: -c();\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // missing error
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.err(
                "@use \"midstream\" as *;\n\
             \n@include -a;\n"
            ),
            "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include -a;\
         \n  | ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.err(
                "@use \"midstream\" as *;\n\
             \na {b: $-c};\n"
            ),
            "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $-c};\
         \n  |       ^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
}
