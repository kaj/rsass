//! Tests auto-converted from "sass-spec/spec/directives/use/error/member/conflict.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("conflict")
        .mock_file(
            "function/other1.scss",
            "@function member() {@return from other1}\n",
        )
        .mock_file(
            "function/other2.scss",
            "@function member() {@return from other2}\n",
        )
        .mock_file("mixin/other1.scss", "@mixin member {a: from other1}\n")
        .mock_file("mixin/other2.scss", "@mixin member {a: from other2}\n")
        .mock_file(
            "same_value/function/_other1.scss",
            "@function c() {@return d}\n",
        )
        .mock_file(
            "same_value/function/_other2.scss",
            "@function c() {@return d}\n",
        )
        .mock_file("same_value/mixin/_other1.scss", "@mixin b {c: d}\n")
        .mock_file("same_value/mixin/_other2.scss", "@mixin b {c: d}\n")
        .mock_file("same_value/variable/_other1.scss", "$c: d;\n")
        .mock_file("same_value/variable/_other2.scss", "$c: d;\n")
        .mock_file("variable/other1.scss", "$member: from other1;\n")
        .mock_file("variable/other2.scss", "$member: from other2;\n")
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
