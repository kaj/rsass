//! Tests auto-converted from "sass-spec/spec/directives/use/error/member.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("member")
        .mock_file(
            "before_use/function/other.scss",
            "@function member() {@return null}\n",
        )
        .mock_file(
            "before_use/variable_declaration/other.scss",
            "$member: value;\n",
        )
        .mock_file(
            "before_use/variable_declaration_without_namespace/other.scss",
            "$member: from other;\n",
        )
        .mock_file("before_use/variable_use/other.scss", "$member: value;\n")
        .mock_file(
            "conflict/function/other1.scss",
            "@function member() {@return from other1}\n",
        )
        .mock_file(
            "conflict/function/other2.scss",
            "@function member() {@return from other2}\n",
        )
        .mock_file(
            "conflict/mixin/other1.scss",
            "@mixin member {a: from other1}\n",
        )
        .mock_file(
            "conflict/mixin/other2.scss",
            "@mixin member {a: from other2}\n",
        )
        .mock_file(
            "conflict/same_value/function/_other1.scss",
            "@function c() {@return d}\n",
        )
        .mock_file(
            "conflict/same_value/function/_other2.scss",
            "@function c() {@return d}\n",
        )
        .mock_file(
            "conflict/same_value/mixin/_other1.scss",
            "@mixin b {c: d}\n",
        )
        .mock_file(
            "conflict/same_value/mixin/_other2.scss",
            "@mixin b {c: d}\n",
        )
        .mock_file("conflict/same_value/variable/_other1.scss", "$c: d;\n")
        .mock_file("conflict/same_value/variable/_other2.scss", "$c: d;\n")
        .mock_file("conflict/variable/other1.scss", "$member: from other1;\n")
        .mock_file("conflict/variable/other2.scss", "$member: from other2;\n")
        .mock_file(
            "inaccessible/private/function/other.scss",
            "@function -member() {@return value}\n",
        )
        .mock_file(
            "inaccessible/private/mixin/other.scss",
            "@mixin -member {a {b: c}}\n",
        )
        .mock_file(
            "inaccessible/private/variable/other.scss",
            "$-member: value;\n",
        )
        .mock_file(
            "inaccessible/transitive/function/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "inaccessible/transitive/function/upstream.scss",
            "@function upstream() {@return value}\n",
        )
        .mock_file(
            "inaccessible/transitive/mixin/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "inaccessible/transitive/mixin/upstream.scss",
            "@mixin upstream {a {b: c}}\n",
        )
        .mock_file(
            "inaccessible/transitive/variable/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "inaccessible/transitive/variable/upstream.scss",
            "$upstream: value;\n",
        )
        .mock_file(
            "inaccessible/transitive_from_import/function/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "inaccessible/transitive_from_import/function/upstream.scss",
            "@function upstream() {@return value}\n",
        )
        .mock_file(
            "inaccessible/transitive_from_import/mixin/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "inaccessible/transitive_from_import/mixin/upstream.scss",
            "@mixin upstream {a {b: c}}\n",
        )
        .mock_file(
            "inaccessible/transitive_from_import/variable/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "inaccessible/transitive_from_import/variable/upstream.scss",
            "$upstream: value;\n",
        )
        .mock_file("missing/global/mixin/other.scss", "")
        .mock_file("missing/global/variable/other.scss", "")
        .mock_file("missing/namespaced/function/other.scss", "")
        .mock_file("missing/namespaced/mixin/other.scss", "")
        .mock_file("missing/namespaced/variable_declaration/other.scss", "")
        .mock_file("missing/namespaced/variable_use/other.scss", "")
}

mod before_use {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("before_use")
    }

    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.err(
                "$variable: other.member();\
             \n@use \"other\";\n"
            ),
            "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | $variable: other.member();\
         \n  |            ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn variable_declaration() {
        let runner = runner().with_cwd("variable_declaration");
        assert_eq!(
            runner.err(
                "other.$member: value;\
             \n@use \"other\";\n"
            ),
            "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | other.$member: value;\
         \n  | ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn variable_declaration_without_namespace() {
        let runner =
            runner().with_cwd("variable_declaration_without_namespace");
        assert_eq!(
        runner.err(
            "$member: from input;\n\
             \n@use \"other\" as *;\n\
             \na {b: $member}\n"
        ),
        "Error: This module and the new module both define a variable named \"$member\".\
         \n  ,\
         \n3 | @use \"other\" as *;\
         \n  | ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
    }
    #[test]
    fn variable_use() {
        let runner = runner().with_cwd("variable_use");
        assert_eq!(
            runner.err(
                "$variable: other.$member;\
             \n@use \"other\";\n"
            ),
            "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | $variable: other.$member;\
         \n  |            ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
mod conflict {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("conflict")
    }

    #[test]
    #[ignore] // missing error
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.err(
                "@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {b: member()}\n"
            ),
            "Error: This function is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes function\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes function\
         \n... |\
         \n4   | a {b: member()}\
         \n    |       ^^^^^^^^ function use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.err(
                "@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {@include member}\n"
            ),
            "Error: This mixin is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes mixin\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes mixin\
         \n... |\
         \n4   | a {@include member}\
         \n    |    ^^^^^^^^^^^^^^^ mixin use\
         \n    \'\
         \n  input.scss 4:4  root stylesheet",
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
            "@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {b: c()}\n"
        ),
        "Error: This function is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes function\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes function\
         \n... |\
         \n4   | a {b: c()}\
         \n    |       ^^^ function use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn mixin() {
            let runner = runner().with_cwd("mixin");
            assert_eq!(
        runner.err(
            "@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {@include b}\n"
        ),
        "Error: This mixin is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes mixin\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes mixin\
         \n... |\
         \n4   | a {@include b}\
         \n    |    ^^^^^^^^^^ mixin use\
         \n    \'\
         \n  input.scss 4:4  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn variable() {
            let runner = runner().with_cwd("variable");
            assert_eq!(
        runner.err(
            "@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {b: $c}\n"
        ),
        "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes variable\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes variable\
         \n... |\
         \n4   | a {b: $c}\
         \n    |       ^^ variable use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.err(
                "@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {b: $member}\n"
            ),
            "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes variable\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes variable\
         \n... |\
         \n4   | a {b: $member}\
         \n    |       ^^^^^^^ variable use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
        );
    }
}
mod inaccessible {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("inaccessible")
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
            "@use \"other\" as *;\n\
             \n// This is technically not a compile error, since `-member()` is treated as\
             \n// plain CSS, but it\'s included here for consistency with the other specs.\
             \na {b: -member()};\n"
        ),
        "a {\
         \n  b: -member();\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // missing error
        fn mixin() {
            let runner = runner().with_cwd("mixin");
            assert_eq!(
                runner.err(
                    "@use \"other\" as *;\n\
             \n@include -member;\n"
                ),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include -member;\
         \n  | ^^^^^^^^^^^^^^^^\
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
                    "@use \"other\" as *;\n\
             \na {b: $-member};\n"
                ),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $-member};\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
    mod transitive {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("transitive")
        }

        #[test]
        #[ignore] // wrong result
        fn function() {
            let runner = runner().with_cwd("function");
            assert_eq!(
        runner.ok(
            "@use \"midstream\" as *;\n\
             \n// This is technically not a compile error, since `-member()` is treated as\
             \n// plain CSS, but it\'s included here for consistency with the other specs.\
             \na {b: upstream()};\n"
        ),
        "a {\
         \n  b: upstream();\
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
             \n@include upstream;\n"
                ),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include upstream;\
         \n  | ^^^^^^^^^^^^^^^^^\
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
             \na {b: $upstream};\n"
                ),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $upstream};\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
    mod transitive_from_import {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("transitive_from_import")
        }

        #[test]
        #[ignore] // wrong result
        fn function() {
            let runner = runner().with_cwd("function");
            assert_eq!(
        runner.ok(
            "@import \"midstream\";\n\
             \n// This is technically not a compile error, since `upstream()` is treated as\
             \n// plain CSS, but it\'s included here for consistency with the other specs.\
             \na {b: upstream()};\n"
        ),
        "a {\
         \n  b: upstream();\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // missing error
        fn mixin() {
            let runner = runner().with_cwd("mixin");
            assert_eq!(
                runner.err(
                    "@import \"midstream\";\n\
             \n@include upstream;\n"
                ),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include upstream;\
         \n  | ^^^^^^^^^^^^^^^^^\
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
                    "@import \"midstream\";\n\
             \na {b: $upstream};\n"
                ),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $upstream};\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
}
mod missing {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("missing")
    }

    mod global {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("global")
        }

        #[test]
        fn mixin() {
            let runner = runner().with_cwd("mixin");
            assert_eq!(
                runner.err(
                    "@use \"other\";\n\
             \n@include member;\n"
                ),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include member;\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        fn variable() {
            let runner = runner().with_cwd("variable");
            assert_eq!(
                runner.err(
                    "@use \"other\";\n\
             \na {b: $member}\n"
                ),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $member}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
    mod namespaced {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("namespaced")
        }

        #[test]
        fn function() {
            let runner = runner().with_cwd("function");
            assert_eq!(
                runner.err(
                    "@use \"other\";\n\
             \na {b: other.member()}\n"
                ),
                "Error: Undefined function.\
         \n  ,\
         \n3 | a {b: other.member()}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
        #[test]
        fn mixin() {
            let runner = runner().with_cwd("mixin");
            assert_eq!(
                runner.err(
                    "@use \"other\";\n\
             \n@include other.member;\n"
                ),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include other.member;\
         \n  | ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        fn variable_declaration() {
            let runner = runner().with_cwd("variable_declaration");
            assert_eq!(
                runner.err(
                    "@use \"other\";\n\
             \nother.$member: value;\n"
                ),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | other.$member: value;\
         \n  | ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        fn variable_use() {
            let runner = runner().with_cwd("variable_use");
            assert_eq!(
                runner.err(
                    "@use \"other\";\n\
             \na {b: other.$member}\n"
                ),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: other.$member}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
}
