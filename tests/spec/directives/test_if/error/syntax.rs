//! Tests auto-converted from "sass-spec/spec/directives/if/error/syntax.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod test_else {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn partial_if() {
        assert_eq!(
        runner().err(
            "// Regression test for sass/dart-sass#1029. The lack of a trailing newline is\
             \n// necessary for the regression test.\
             \n@if a {}\
             \n@else i"
        ),
        "Error: expected \"{\".\
         \n  ,\
         \n4 | @else i\
         \n  |       ^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
    );
    }
}
