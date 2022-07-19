//! Tests auto-converted from "sass-spec/spec/css/selector/pseudoselector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("pseudoselector")
}

mod nested {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn adjacent_combinators() {
        assert_eq!(
            runner().ok("// Regression test for sass/dart-sass#1038\
             \na {\
             \n  b:c, > d {x: y}\
             \n}\n"),
            "a b:c, a > d {\
         \n  x: y;\
         \n}\n"
        );
    }
}
