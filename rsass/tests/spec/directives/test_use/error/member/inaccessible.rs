//! Tests auto-converted from "sass-spec/spec/directives/use/error/member/inaccessible.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("inaccessible")
        .mock_file(
            "private/function/other.scss",
            "@function -member() {@return value}\n",
        )
        .mock_file("private/mixin/other.scss", "@mixin -member {a {b: c}}\n")
        .mock_file("private/variable/other.scss", "$-member: value;\n")
        .mock_file(
            "transitive/function/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "transitive/function/upstream.scss",
            "@function upstream() {@return value}\n",
        )
        .mock_file(
            "transitive/mixin/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "transitive/mixin/upstream.scss",
            "@mixin upstream {a {b: c}}\n",
        )
        .mock_file(
            "transitive/variable/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file("transitive/variable/upstream.scss", "$upstream: value;\n")
        .mock_file(
            "transitive_from_import/function/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "transitive_from_import/function/upstream.scss",
            "@function upstream() {@return value}\n",
        )
        .mock_file(
            "transitive_from_import/mixin/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "transitive_from_import/mixin/upstream.scss",
            "@mixin upstream {a {b: c}}\n",
        )
        .mock_file(
            "transitive_from_import/variable/midstream.scss",
            "@use \"upstream\" as *;\n",
        )
        .mock_file(
            "transitive_from_import/variable/upstream.scss",
            "$upstream: value;\n",
        )
}

mod private {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("private")
    }

    #[test]
    #[ignore] // wrong result
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
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"midstream\";\
         \n  |         ^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: Undefined mixin.\
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
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"midstream\";\
         \n  |         ^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: Undefined variable.\
         \n  ,\
         \n3 | a {b: $upstream};\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
}
