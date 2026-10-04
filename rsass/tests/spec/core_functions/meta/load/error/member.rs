//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load/error/member.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("member")
        .mock_file("global/_other.scss", "$c: d;\n")
        .mock_file("namespace/_other.scss", "$c: d;\n")
}

#[test]
#[ignore] // wrong error
fn global() {
    let runner = runner().with_cwd("global");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n$_: meta.load(\"other\");\
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
#[test]
#[ignore] // wrong error
fn namespace() {
    let runner = runner().with_cwd("namespace");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n$_: meta.load(\"other\");\
             \na {b: other.$c}\n"
        ),
        "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n3 | a {b: other.$c}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
}
