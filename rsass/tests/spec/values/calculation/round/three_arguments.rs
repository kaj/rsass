//! Tests auto-converted from "sass-spec/spec/values/calculation/round/three_arguments.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("three_arguments")
}

mod step {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn unknown_variable() {
        assert_eq!(
            runner().ok("a {\
             \n  d: round(up, 8px, var(--c));\
             \n}"),
            "a {\
         \n  d: round(up, 8px, var(--c));\
         \n}\n"
        );
    }
}
mod strategy {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn interpolation() {
        assert_eq!(
            runner().ok("a {\
             \n  e: round(#{\"up\"}, 3px, 9px);\
             \n}\n"),
            "a {\
         \n  e: 9px;\
         \n}\n"
        );
    }
    #[test]
    fn unknown_variable() {
        assert_eq!(
            runner().ok("a {\
             \n  e: round(var(--c), 8px, 9px);\
             \n}"),
            "a {\
         \n  e: round(var(--c), 8px, 9px);\
         \n}\n"
        );
    }
}
mod units {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn real_and_unknown() {
        assert_eq!(
            runner().ok("a {b: round(nearest, 1px, 10%)}\n"),
            "a {\
         \n  b: round(nearest, 1px, 10%);\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {\
             \n  b: round(nearest, 1%, 2%);\
             \n}\n"),
            "a {\
         \n  b: 2%;\
         \n}\n"
        );
    }
}
