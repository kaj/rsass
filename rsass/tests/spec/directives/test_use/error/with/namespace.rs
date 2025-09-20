//! Tests auto-converted from "sass-spec/spec/directives/use/error/with/namespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("namespace")
        .mock_file(
            "_midstream.scss",
            "@use \"upstream\";\nupstream.$a: c !default;\n",
        )
        .mock_file("_upstream.scss", "$a: d;\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"midstream\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"midstream\" with ($a: b);\
         \n  |                        ^^^^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
    );
}
