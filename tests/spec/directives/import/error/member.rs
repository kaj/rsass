//! Tests auto-converted from "sass-spec/spec/directives/import/error/member.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("member")
        .mock_file(
            "inaccessible/nested/function/_other.scss",
            "@function d() {@return e}\n",
        )
        .mock_file(
            "inaccessible/nested/mixin/_other.scss",
            "@mixin c() {d: e};\n",
        )
        .mock_file("inaccessible/nested/variable/_other.scss", "$d: e;\n")
}

mod inaccessible {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("inaccessible")
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
                runner.ok("a {@import \"other\"}\n\
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
                    "a {@import \"other\"}\n\
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
        fn variable() {
            let runner = runner().with_cwd("variable");
            assert_eq!(
                runner.err(
                    "a {@import \"other\"}\n\
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
