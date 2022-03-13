//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/extend.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("in_input/after/_other.scss", "a {b: c}\n")
        .mock_file("in_input/before/_other.scss", "a {b: c}\n")
        .mock_file("in_other/after/_other.scss", "d {@extend a !optional}\n")
        .mock_file("in_other/before/_other.scss", "d {@extend a !optional}\n")
}

mod in_input {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("in_input")
    }

    #[test]
    #[ignore] // wrong result
    fn after() {
        let runner = runner().with_cwd("after");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n\
             \nd {@extend a}\n"),
            "a, d {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn before() {
        let runner = runner().with_cwd("before");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\n\
             \nd {@extend a}\
             \n@include meta.load-css(\"other\");\n"),
            "a, d {\
         \n  b: c;\
         \n}\n"
        );
    }
}
mod in_other {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("in_other")
    }

    #[test]
    #[ignore] // wrong result
    fn after() {
        let runner = runner().with_cwd("after");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\n\
             \n@include meta.load-css(\"other\");\
             \na {b: c}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn before() {
        let runner = runner().with_cwd("before");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\n\
             \na {b: c}\
             \n@include meta.load-css(\"other\");\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
