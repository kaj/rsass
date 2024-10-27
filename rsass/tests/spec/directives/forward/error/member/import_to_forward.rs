//! Tests auto-converted from "sass-spec/spec/directives/forward/error/member/import_to_forward.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("import_to_forward")
        .mock_file(
            "nested/function/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "nested/function/_upstream.scss",
            "@function d() {@return e}\n",
        )
        .mock_file("nested/mixin/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("nested/mixin/_upstream.scss", "@mixin c {d: e}\n")
        .mock_file(
            "nested/variable/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file("nested/variable/_upstream.scss", "$d: e;\n")
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
        "DEPRECATION WARNING: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {@import \"midstream\"}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:12  root stylesheet\n\
         \nError: Undefined mixin.\
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
        "DEPRECATION WARNING: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {@import \"midstream\"}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:12  root stylesheet\n\
         \nError: Undefined variable.\
         \n  ,\
         \n3 | b {c: $d}\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
}
