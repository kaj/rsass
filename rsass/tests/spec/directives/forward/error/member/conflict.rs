//! Tests auto-converted from "sass-spec/spec/directives/forward/error/member/conflict.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("conflict")
        .mock_file("because_of_as/first/_other1.scss", "$b: from other1;\n")
        .mock_file("because_of_as/first/_other2.scss", "$a-b: from other2;\n")
        .mock_file("because_of_as/last/_other1.scss", "$a-b: from other1;\n")
        .mock_file("because_of_as/last/_other2.scss", "$b: from other2;\n")
        .mock_file(
            "function/_other1.scss",
            "@function a() {@return from other1}\n",
        )
        .mock_file(
            "function/_other2.scss",
            "@function a() {@return from other2}\n",
        )
        .mock_file("mixin/_other1.scss", "@mixin a {b: from other1}\n")
        .mock_file("mixin/_other2.scss", "@mixin a {b: from other2}\n")
        .mock_file(
            "same_value/function/_other1.scss",
            "@function a() {@return b}\n",
        )
        .mock_file(
            "same_value/function/_other2.scss",
            "@function a() {@return b}\n",
        )
        .mock_file("same_value/mixin/_other1.scss", "@mixin a {b: c}\n")
        .mock_file("same_value/mixin/_other2.scss", "@mixin a {b: c}\n")
        .mock_file("same_value/variable/_other1.scss", "$a: b;\n")
        .mock_file("same_value/variable/_other2.scss", "$a: b;\n")
        .mock_file("variable/_other1.scss", "$a: from other1;\n")
        .mock_file("variable/_other2.scss", "$a: from other2;\n")
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
