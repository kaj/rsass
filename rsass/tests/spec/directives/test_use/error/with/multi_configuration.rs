//! Tests auto-converted from "sass-spec/spec/directives/use/error/with/multi_configuration.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("multi_configuration")
        .mock_file("multi_file/_left.scss", "@use \"other\" with ($a: b);\n")
        .mock_file("multi_file/_other.scss", "$a: c !default;\n")
        .mock_file("multi_file/_right.scss", "@use \"other\" with ($a: b);\n")
        .mock_file("one_file/_other.scss", "$a: c !default;\n")
        .mock_file("through_forward/_forwarded.scss", "// This only throws an error because it defines a configurable variable.\n$c: f !default;\n")
        .mock_file("through_forward/_midstream.scss", "@forward \"forwarded\";\n\n$a: e !default;\n")
        .mock_file("unconfigured_first/_other.scss", "$a: c !default;\n")
}

#[test]
#[ignore] // missing error
fn multi_file() {
    let runner = runner().with_cwd("multi_file");
    assert_eq!(
        runner.err(
            "@use \"left\";\
             \n@use \"right\";\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _right.scss\
         \n1 | @use \"other\" with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> _left.scss\
         \n1 | @use \"other\" with ($a: b);\
         \n  | ========================= original load\
         \n  \'\
         \n  _right.scss 1:1  @use\
         \n  input.scss 2:1   root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn one_file() {
    let runner = runner().with_cwd("one_file");
    assert_eq!(
        runner.err(
            "@use \"other\" as o1 with ($a: b);\
             \n@use \"other\" as o2 with ($a: b);\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n1 | @use \"other\" as o1 with ($a: b);\
         \n  | =============================== original load\
         \n2 | @use \"other\" as o2 with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn through_forward() {
    let runner = runner().with_cwd("through_forward");
    assert_eq!(
        runner.err(
            "@use \"forwarded\";\
             \n@use \"midstream\" with ($a: b, $c: d);\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _midstream.scss\
         \n1 | @forward \"forwarded\";\
         \n  | ^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n1 | @use \"forwarded\";\
         \n  | ================ original load\
         \n2 | @use \"midstream\" with ($a: b, $c: d);\
         \n  | ==================================== configuration\
         \n  \'\
         \n  _midstream.scss 1:1  @use\
         \n  input.scss 2:1       root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn unconfigured_first() {
    let runner = runner().with_cwd("unconfigured_first");
    assert_eq!(
        runner.err(
            "@use \"other\" as o1;\
             \n@use \"other\" as o2 with ($a: b);\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n1 | @use \"other\" as o1;\
         \n  | ================== original load\
         \n2 | @use \"other\" as o2 with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
