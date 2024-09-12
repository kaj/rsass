//! Tests auto-converted from "sass-spec/spec/directives/warn.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("warn")
}

mod comment {
    #[allow(unused)]
    use super::runner;

    mod after_expression {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@warn a /**/\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(runner().ok("@warn a //\n"), "");
        }
    }
    mod before_expression {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@warn /**/ a\n"), "");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@warn //\
             \n  a\n"),
                ""
            );
        }
    }
}
#[test]
fn escaped() {
    assert_eq!(
        runner().ok("@w\\61rn warning;\
             \na {b: c}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn functions_in_stack() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \n@function issues-warning($a) {\
             \n  @warn \"From function: #{meta.inspect($a)}\";\
             \n  @return $a;\
             \n}\n\
             \n@mixin calls-function-that-warns($a) {\
             \n  warned: issues-warning($a);\
             \n}\n\
             \n.test {\
             \n  @include calls-function-that-warns(testing);\
             \n}\n"),
        ".test {\
         \n  warned: testing;\
         \n}\n"
    );
}
mod position {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn function() {
        assert_eq!(
            runner().ok("@function c() {\
             \n  @warn test;\
             \n  @return d;\
             \n}\n\
             \na {\
             \n  b: c();\
             \n}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    fn mixin() {
        assert_eq!(
            runner().ok("@mixin b() {\
             \n  @warn test;\
             \n  c: d;\
             \n}\n\
             \na {\
             \n  @include b();\
             \n}\n"),
            "a {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    fn property() {
        assert_eq!(
            runner().ok("a {\
             \n  b: {\
             \n    @warn \"w\";\
             \n    c: d;\
             \n  }\
             \n}\n"),
            "a {\
         \n  b-c: d;\
         \n}\n"
        );
    }
    #[test]
    fn ruleset() {
        assert_eq!(
            runner().ok("a {\
             \n  @warn \"w\";\
             \n  b: c;\
             \n}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn top_level() {
        assert_eq!(runner().ok("@warn \"w\";\n"), "");
    }
}
