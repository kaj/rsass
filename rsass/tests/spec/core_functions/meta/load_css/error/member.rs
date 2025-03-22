//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/member.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("member")
        .mock_file("global/_other.scss", "$c: d;\n")
        .mock_file("namespace/_other.scss", "$c: d;\n")
}

#[test]
fn global() {
    let runner = runner().with_cwd("global");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@meta.load-css(\"other\");\n\
             \na {b: $c}\n"
        ),
        "Error: Undefined variable.\
         \n  ,\
         \n4 | a {b: $c}\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
    );
}
#[test]
fn namespace() {
    let runner = runner().with_cwd("namespace");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@meta.load-css(\"other\");\n\
             \na {b: other.$c}\n"
        ),
        "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n4 | a {b: other.$c}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
    );
}
