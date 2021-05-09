//! Tests auto-converted from "sass-spec/spec/css/keyframes.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod bubble {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // wrong result
    fn empty() {
        assert_eq!(
            runner().ok("// Regression test for sass/dart-sass#611.\
             \na {\
             \n  @keyframes {/**/}\
             \n}\n"),
            "@keyframes {\
         \n  /**/\
         \n}\n"
        );
    }
}
