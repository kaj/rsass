//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/with/conflict.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("conflict")
        .mock_file("_left.scss", "$a: left;\n")
        .mock_file(
            "_midstream.scss",
            "@use \"left\" as *;\n@use \"right\" as *;\n\n$a: c !default;\n",
        )
        .mock_file("_right.scss", "$a: right;\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\n"
        ),
        "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"left\" as *;\
         \n    | ================ includes variable\
         \n2   | @use \"right\" as *;\
         \n    | ================= includes variable\
         \n... |\
         \n4   | $a: c !default;\
         \n    | ^^^^^^^^^^^^^^ variable use\
         \n    \'\
         \n  _midstream.scss 4:1  load-css()\
         \n  input.scss 2:1       root stylesheet",
    );
}
