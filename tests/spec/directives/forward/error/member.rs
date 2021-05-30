//! Tests auto-converted from "sass-spec/spec/directives/forward/error/member.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file(
            "conflict/because_of_as/first/_other1.scss",
            "$b: from other1;\n",
        )
        .mock_file(
            "conflict/because_of_as/first/_other2.scss",
            "$a-b: from other2;\n",
        )
        .mock_file(
            "conflict/because_of_as/last/_other1.scss",
            "$a-b: from other1;\n",
        )
        .mock_file(
            "conflict/because_of_as/last/_other2.scss",
            "$b: from other2;\n",
        )
        .mock_file(
            "conflict/function/_other1.scss",
            "@function a() {@return from other1}\n",
        )
        .mock_file(
            "conflict/function/_other2.scss",
            "@function a() {@return from other2}\n",
        )
        .mock_file(
            "conflict/mixin/_other1.scss",
            "@mixin a {b: from other1}\n",
        )
        .mock_file(
            "conflict/mixin/_other2.scss",
            "@mixin a {b: from other2}\n",
        )
        .mock_file(
            "conflict/same_value/function/_other1.scss",
            "@function a() {@return b}\n",
        )
        .mock_file(
            "conflict/same_value/function/_other2.scss",
            "@function a() {@return b}\n",
        )
        .mock_file(
            "conflict/same_value/mixin/_other1.scss",
            "@mixin a {b: c}\n",
        )
        .mock_file(
            "conflict/same_value/mixin/_other2.scss",
            "@mixin a {b: c}\n",
        )
        .mock_file("conflict/same_value/variable/_other1.scss", "$a: b;\n")
        .mock_file("conflict/same_value/variable/_other2.scss", "$a: b;\n")
        .mock_file("conflict/variable/_other1.scss", "$a: from other1;\n")
        .mock_file("conflict/variable/_other2.scss", "$a: from other2;\n")
        .mock_file(
            "import_to_forward/nested/function/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "import_to_forward/nested/function/_upstream.scss",
            "@function d() {@return e}\n",
        )
        .mock_file(
            "import_to_forward/nested/mixin/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "import_to_forward/nested/mixin/_upstream.scss",
            "@mixin c {d: e}\n",
        )
        .mock_file(
            "import_to_forward/nested/variable/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "import_to_forward/nested/variable/_upstream.scss",
            "$d: e;\n",
        )
        .mock_file(
            "inaccessible/hidden/as/different_separator/_midstream.scss",
            "@forward \"upstream\" as b-* hide b_a;\n",
        )
        .mock_file(
            "inaccessible/hidden/as/different_separator/_upstream.scss",
            "@as a {b {c: d}}\n",
        )
        .mock_file(
            "inaccessible/hidden/as/same_separator/_midstream.scss",
            "@forward \"upstream\" as b-* hide b-a;\n",
        )
        .mock_file(
            "inaccessible/hidden/as/same_separator/_upstream.scss",
            "@as a {b {c: d}}\n",
        )
        .mock_file(
            "inaccessible/hidden/mixin/_midstream.scss",
            "@forward \"upstream\" hide a;\n",
        )
        .mock_file(
            "inaccessible/hidden/mixin/_upstream.scss",
            "@mixin a {b {c: d}}\n",
        )
        .mock_file(
            "inaccessible/hidden/variable/_midstream.scss",
            "@forward \"upstream\" hide $c;\n",
        )
        .mock_file("inaccessible/hidden/variable/_upstream.scss", "$c: d;\n")
        .mock_file(
            "inaccessible/local/function/_other.scss",
            "@function c() {@return d}\n",
        )
        .mock_file(
            "inaccessible/local/mixin/_other.scss",
            "@mixin a {b {c: d}}\n",
        )
        .mock_file("inaccessible/local/variable/_other.scss", "$c: d;\n")
        .mock_file(
            "inaccessible/not_shown/as/mixin/_midstream.scss",
            "@forward \"upstream\" as b-* show a;\n",
        )
        .mock_file(
            "inaccessible/not_shown/as/mixin/_upstream.scss",
            "@mixin a {c {d: e}}\n",
        )
        .mock_file(
            "inaccessible/not_shown/mixin/_midstream.scss",
            "@forward \"upstream\" show b;\n",
        )
        .mock_file(
            "inaccessible/not_shown/mixin/_upstream.scss",
            "@mixin a {c {d: e}}\n",
        )
        .mock_file(
            "inaccessible/not_shown/variable/_midstream.scss",
            "@forward \"upstream\" show $d;\n",
        )
        .mock_file(
            "inaccessible/not_shown/variable/_upstream.scss",
            "$c: e;\n",
        )
        .mock_file(
            "inaccessible/not_shown/wrong_type/mixin/_midstream.scss",
            "@forward \"upstream\" show $a;\n",
        )
        .mock_file(
            "inaccessible/not_shown/wrong_type/mixin/_upstream.scss",
            "@mixin a {c {d: e}}\n",
        )
        .mock_file(
            "inaccessible/not_shown/wrong_type/variable/_midstream.scss",
            "@forward \"upstream\" show c;\n",
        )
        .mock_file(
            "inaccessible/not_shown/wrong_type/variable/_upstream.scss",
            "$c: e;\n",
        )
        .mock_file(
            "inaccessible/private/function/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "inaccessible/private/function/_upstream.scss",
            "@function -c() {@return d}\n",
        )
        .mock_file(
            "inaccessible/private/mixin/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "inaccessible/private/mixin/_upstream.scss",
            "@mixin -a {b {c: d}}\n",
        )
        .mock_file(
            "inaccessible/private/variable/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "inaccessible/private/variable/_upstream.scss",
            "$-c: d;\n",
        )
}

mod conflict {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("conflict")
    }

    mod because_of_as {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("because_of_as")
        }

        #[test]
        #[ignore] // missing error
        fn first() {
            let runner = runner().with_cwd("first");
            assert_eq!(
        runner.err(
            "@forward \"other1\" as a-*;\
             \n@forward \"other2\";\n"
        ),
        "Error: Two forwarded modules both define a variable named $a-b.\
         \n  ,\
         \n1 | @forward \"other1\" as a-*;\
         \n  | ======================== original @forward\
         \n2 | @forward \"other2\";\
         \n  | ^^^^^^^^^^^^^^^^^ new @forward\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn last() {
            let runner = runner().with_cwd("last");
            assert_eq!(
        runner.err(
            "@forward \"other1\";\
             \n@forward \"other2\" as a-*;\n"
        ),
        "Error: Two forwarded modules both define a variable named $a-b.\
         \n  ,\
         \n1 | @forward \"other1\";\
         \n  | ================= original @forward\
         \n2 | @forward \"other2\" as a-*;\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^ new @forward\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.err(
                "@forward \"other1\";\
             \n@forward \"other2\";\n"
            ),
            "Error: Two forwarded modules both define a function named a.\
         \n  ,\
         \n1 | @forward \"other1\";\
         \n  | ================= original @forward\
         \n2 | @forward \"other2\";\
         \n  | ^^^^^^^^^^^^^^^^^ new @forward\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.err(
                "@forward \"other1\";\
             \n@forward \"other2\";\n"
            ),
            "Error: Two forwarded modules both define a mixin named a.\
         \n  ,\
         \n1 | @forward \"other1\";\
         \n  | ================= original @forward\
         \n2 | @forward \"other2\";\
         \n  | ^^^^^^^^^^^^^^^^^ new @forward\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    mod same_value {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("same_value")
        }

        #[test]
        #[ignore] // missing error
        fn function() {
            let runner = runner().with_cwd("function");
            assert_eq!(
        runner.err(
            "@forward \"other1\";\
             \n@forward \"other2\";\n"
        ),
        "Error: Two forwarded modules both define a function named a.\
         \n  ,\
         \n1 | @forward \"other1\";\
         \n  | ================= original @forward\
         \n2 | @forward \"other2\";\
         \n  | ^^^^^^^^^^^^^^^^^ new @forward\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn mixin() {
            let runner = runner().with_cwd("mixin");
            assert_eq!(
                runner.err(
                    "@forward \"other1\";\
             \n@forward \"other2\";\n"
                ),
                "Error: Two forwarded modules both define a mixin named a.\
         \n  ,\
         \n1 | @forward \"other1\";\
         \n  | ================= original @forward\
         \n2 | @forward \"other2\";\
         \n  | ^^^^^^^^^^^^^^^^^ new @forward\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn variable() {
            let runner = runner().with_cwd("variable");
            assert_eq!(
        runner.err(
            "@forward \"other1\";\
             \n@forward \"other2\";\n"
        ),
        "Error: Two forwarded modules both define a variable named $a.\
         \n  ,\
         \n1 | @forward \"other1\";\
         \n  | ================= original @forward\
         \n2 | @forward \"other2\";\
         \n  | ^^^^^^^^^^^^^^^^^ new @forward\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.err(
                "@forward \"other1\";\
             \n@forward \"other2\";\n"
            ),
            "Error: Two forwarded modules both define a variable named $a.\
         \n  ,\
         \n1 | @forward \"other1\";\
         \n  | ================= original @forward\
         \n2 | @forward \"other2\";\
         \n  | ^^^^^^^^^^^^^^^^^ new @forward\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
mod import_to_forward {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("import_to_forward")
    }

    mod nested {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("nested")
        }

        #[test]
        fn function() {
            let runner = runner().with_cwd("function");
            assert_eq!(
                runner.ok("a {@import \"midstream\"}\n\
             \nb {c: d()}\n"),
                "b {\
         \n  c: d();\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong error
        fn mixin() {
            let runner = runner().with_cwd("mixin");
            assert_eq!(
                runner.err(
                    "a {@import \"midstream\"}\n\
             \nb {@include c}\n"
                ),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | b {@include c}\
         \n  |    ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:4  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn variable() {
            let runner = runner().with_cwd("variable");
            assert_eq!(
                runner.err(
                    "a {@import \"midstream\"}\n\
             \nb {c: $d}\n"
                ),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | b {c: $d}\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
}
mod inaccessible {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("inaccessible")
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
            #[ignore] // wrong error
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
            #[ignore] // wrong error
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
        #[ignore] // wrong error
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
        #[ignore] // wrong error
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
        #[ignore] // wrong error
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
        #[ignore] // wrong error
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
            #[ignore] // missing error
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
        #[ignore] // wrong error
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
        #[ignore] // wrong error
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
            #[ignore] // wrong error
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
            #[ignore] // wrong error
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
}
