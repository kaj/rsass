//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/four_args/alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod percent {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn above_max() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 0%, 0%, 250%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 0%, 0%, 100%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 0%, 0%, 0%)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 0%, 0%, -10%)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 0%, 0%, 45.6%)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0.456);\
         \n}\n"
        );
    }
}
mod unitless {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn above_max() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 0%, 0%, 250)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 0%, 0%, 1)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 0%, 0%, 0)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 0%, 0%, -10)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 0%, 0%, 0.456)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0.456);\
         \n}\n"
        );
    }
}
