//! Tests auto-converted from "sass-spec/spec/variables/semi_global.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("semi_global")
}

mod in_local {
    use super::runner;

    #[test]
    fn double_nested() {
        assert_eq!(
            runner().ok("// Regression test for sass/dart-sass#1250\
             \n$a: global;\
             \nb {\
             \n  @if true {\
             \n    @if true {\
             \n      $a: local;\
             \n    }\
             \n  }\
             \n}\n\
             \nc {d: $a}\n"),
            "c {\
         \n  d: global;\
         \n}\n"
        );
    }
}
