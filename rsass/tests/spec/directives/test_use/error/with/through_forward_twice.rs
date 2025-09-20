//! Tests auto-converted from "sass-spec/spec/directives/use/error/with/through_forward_twice.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("through_forward_twice")
        .mock_file("with/_forwarded.scss", "$a: forwarded a !default;\n$b: forwarded b !default;\n")
        .mock_file("with/_used.scss", "@forward \"forwarded\" with ($a: used a 1 !default);\n@forward \"forwarded\" with ($a: used a 2 !default);\n")
}

#[test]
#[ignore] // missing error
fn with() {
    let runner = runner().with_cwd("with");
    assert_eq!(
        runner.err(
            "@use \"used\" with ($a: input a);\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n1 | @forward \"forwarded\" with ($a: used a 1 !default);\
         \n  | ================================================= original load\
         \n2 | @forward \"forwarded\" with ($a: used a 2 !default);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  _used.scss 2:1  @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
