//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load/error/from_other.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("from_other")
        .mock_file("runtime/_other.scss", "a {b: 1px + 1em}\n")
        .mock_file("syntax/_other.scss", "a {b: }\n")
}

#[test]
#[ignore] // wrong error
fn runtime() {
    let runner = runner().with_cwd("runtime");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n$_: meta.load(\"other\");\n"
        ),
        "Error: 1px and 1em have incompatible units.\
         \n  ,\
         \n1 | a {b: 1px + 1em}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  _other.scss 1:7  load()\
         \n  input.scss 2:5   root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn syntax() {
    let runner = runner().with_cwd("syntax");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n$_: meta.load(\"other\");\n"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n1 | a {b: }\
         \n  |       ^\
         \n  \'\
         \n  _other.scss 1:7  load()\
         \n  input.scss 2:5   root stylesheet",
    );
}
