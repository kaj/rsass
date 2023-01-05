//! Tests auto-converted from "sass-spec/spec/css/media/indentation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("indentation")
}

#[test]
#[ignore] // wrong result
fn media_nested_in_selector() {
    assert_eq!(
        runner().ok("a,\
             \nb {\
             \n  @media c {\
             \n    d,\
             \n    e { f: g }\
             \n  }\
             \n}"),
        "@media c {\
         \n  a d,\
         \n  a e,\
         \n  b d,\
         \n  b e {\
         \n    f: g;\
         \n  }\
         \n}\n"
    );
}
mod nested_selector {
    #[allow(unused)]
    use super::runner;

    mod different_lines_parent {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn different_lines() {
            assert_eq!(
                runner().ok("@media a {\
             \n  b,\
             \n  a {\
             \n    c,\
             \n    d { e: f }\
             \n  }\
             \n}"),
                "@media a {\
         \n  b c,\
         \n  b d,\
         \n  a c,\
         \n  a d {\
         \n    e: f;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn same_line() {
            assert_eq!(
                runner().ok("@media a {\
             \n  b,\
             \n  a {\
             \n    c, d { e: f }\
             \n  }\
             \n}"),
                "@media a {\
         \n  b c, b d,\
         \n  a c,\
         \n  a d {\
         \n    e: f;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod same_lines_parent {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn different_lines() {
            assert_eq!(
                runner().ok("@media a {\
             \n  b, a {\
             \n    c,\
             \n    d { e: f }\
             \n  }\
             \n}"),
                "@media a {\
         \n  b c,\
         \n  b d, a c,\
         \n  a d {\
         \n    e: f;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn same_line() {
            assert_eq!(
                runner().ok("@media a {\
             \n  b, a {\
             \n    c, d { e: f }\
             \n  }\
             \n}"),
                "@media a {\
         \n  b c, b d, a c, a d {\
         \n    e: f;\
         \n  }\
         \n}\n"
            );
        }
    }
}
#[test]
#[ignore] // wrong result
fn simple_selector_on_different_lines() {
    assert_eq!(
        runner().ok("@media a {\
             \n  b,\
             \n  a { c: d }\
             \n}"),
        "@media a {\
         \n  b,\
         \n  a {\
         \n    c: d;\
         \n  }\
         \n}\n"
    );
}
