//! Tests auto-converted from "sass-spec/spec/directives/forward/error/extend.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("_upstream.scss", "in-upstream {@extend in-input}\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"midstream\";\n\
             \nin-input {a: b}\n"
        ),
        "Error: The target selector was not found.\
         \nUse \"@extend in-input !optional\" to avoid this error.\
         \n  ,\
         \n1 | in-upstream {@extend in-input}\
         \n  |              ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _upstream.scss 1:14  root stylesheet",
    );
}
