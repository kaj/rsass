//! Tests auto-converted from "sass-spec/spec/css/custom_properties/trailing_whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("trailing_whitespace")
}

mod scss {
    use super::runner;

    #[test]
    fn before_closing_brace() {
        assert_eq!(
            runner().ok("a {\
             \n  --b: c\
             \n}\n"),
            "a {\
         \n  --b: c ;\
         \n}\n"
        );
    }
    #[test]
    fn newline() {
        assert_eq!(
            runner().ok("a {\
             \n  --b: c\
             \n;\
             \n}\n"),
            "a {\
         \n  --b: c ;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("a {\
             \n  --b: c ;\
             \n}\n"),
            "a {\
         \n  --b: c ;\
         \n}\n"
        );
    }
    #[test]
    fn tab() {
        assert_eq!(
            runner().ok("a {\
             \n  --b: c\t;\
             \n}\n"),
            "a {\
         \n  --b: c\t;\
         \n}\n"
        );
    }
}
