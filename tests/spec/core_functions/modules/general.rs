//! Tests auto-converted from "sass-spec/spec/core_functions/modules/general.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("general")
        .mock_file(
            "forward/as/_other.scss",
            "@forward \"sass:math\" as s-*;\n",
        )
        .mock_file("forward/bare/_other.scss", "@forward \"sass:math\";\n")
        .mock_file(
            "forward/error/hide/_other.scss",
            "@forward \"sass:math\" hide round;\n",
        )
        .mock_file(
            "forward/error/show/_other.scss",
            "@forward \"sass:math\" show ceil;\n",
        )
        .mock_file(
            "forward/hide/_other.scss",
            "@forward \"sass:math\" hide ceil;\n",
        )
        .mock_file(
            "forward/show/_other.scss",
            "@forward \"sass:math\" show round;\n",
        )
}

#[test]
fn test_as() {
    let runner = runner().with_cwd("as");
    assert_eq!(
        runner.ok("@use \"sass:math\" as m;\
             \na {b: m.round(0.7)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    #[ignore] // wrong error
    fn set_variable() {
        let runner = runner().with_cwd("set_variable");
        assert_eq!(
            runner.err(
                "@use \"sass:math\";\
             \nmath.$a: b;\n"
            ),
            "Error: Undefined variable.\
         \n  ,\
         \n2 | math.$a: b;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
mod forward {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("forward")
    }

    #[test]
    fn test_as() {
        let runner = runner().with_cwd("as");
        assert_eq!(
            runner.ok("@use \"other\";\
             \na {b: other.s-round(0.7)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    fn bare() {
        let runner = runner().with_cwd("bare");
        assert_eq!(
            runner.ok("@use \"other\";\
             \na {b: other.round(0.7)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    mod error {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("error")
        }

        #[test]
        fn hide() {
            let runner = runner().with_cwd("hide");
            assert_eq!(
                runner.err(
                    "@use \"other\";\
             \na {b: other.round(0.7)}\n"
                ),
                "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: other.round(0.7)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn show() {
            let runner = runner().with_cwd("show");
            assert_eq!(
                runner.err(
                    "@use \"other\";\
             \na {b: other.round(0.7)}\n"
                ),
                "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: other.round(0.7)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
            runner.ok("@use \"other\";\
             \na {b: other.round(0.7)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
            runner.ok("@use \"other\";\
             \na {b: other.round(0.7)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
}
#[test]
fn global() {
    let runner = runner().with_cwd("global");
    assert_eq!(
        runner.ok("@use \"sass:math\" as *;\
             \na {b: compatible(1px, 1in)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
