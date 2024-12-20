//! Tests auto-converted from "sass-spec/spec/variables.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("variables")
}

mod comment {
    #[allow(unused)]
    use super::runner;

    mod after_colon {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("$a: /**/ b\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("$a: //\
             \n  b\n"),
                ""
            );
        }
    }
    mod after_value {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("$a: b /**/\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(runner().ok("$a: b //\n"), "");
        }
    }
    mod before_colon {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            assert_eq!(runner().ok("$a /**/: b\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("$a //\
             \n  : b\n"),
                ""
            );
        }
    }
}
mod double_flag {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn default() {
        assert_eq!(
            runner().ok("$a: b !default !default;\n\
             \nc {d: $a}\n"),
            "c {\
         \n  d: b;\
         \n}\n"
        );
    }
    #[test]
    fn global() {
        assert_eq!(
            runner().ok("$a: b;\
             \nc {\
             \n  $a: d !global !global;\
             \n  e: $a\
             \n}\n"),
            "c {\
         \n  e: d;\
         \n}\n"
        );
    }
}
mod semi_global {
    #[allow(unused)]
    use super::runner;

    mod in_local {
        #[allow(unused)]
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
}
