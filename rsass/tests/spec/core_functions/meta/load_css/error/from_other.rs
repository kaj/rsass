//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/from_other.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("from_other")
        .mock_file("extend/_other.scss", "a {@extend missing}\n")
        .mock_file("runtime/_other.scss", "a {b: 1px + 1em}\n")
        .mock_file("syntax/_other.scss", "a {b: }\n")
}

#[test]
#[ignore] // wrong error
fn extend() {
    let runner = runner().with_cwd("extend");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"
        ),
        "Error: The target selector was not found.\
         \nUse \"@extend missing !optional\" to avoid this error.\
         \n  ,\
         \n1 | a {@extend missing}\
         \n  |    ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _other.scss 1:4  load-css()\
         \n  input.scss 2:1   root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn runtime() {
    let runner = runner().with_cwd("runtime");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"
        ),
        "Error: 1px and 1em have incompatible units.\
         \n  ,\
         \n1 | a {b: 1px + 1em}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  _other.scss 1:7  load-css()\
         \n  input.scss 2:1   root stylesheet",
    );
}
#[test]
fn syntax() {
    let runner = runner().with_cwd("syntax");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n1 | a {b: }\
         \n  |       ^\
         \n  \'\
         \n  _other.scss 1:7  load-css()\
         \n  input.scss 2:1   root stylesheet",
    );
}
