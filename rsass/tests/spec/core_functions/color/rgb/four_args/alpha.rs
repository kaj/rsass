//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/four_args/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

mod percent {
    use super::runner;

    mod above {
        use super::runner;

        #[test]
        fn max() {
            assert_eq!(
                runner().ok("a {b: rgb(0, 0, 0, 250%)}\n"),
                "a {\
         \n  b: rgb(0, 0, 0);\
         \n}\n"
            );
        }
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 0, 100%)}\n"),
            "a {\
         \n  b: rgb(0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 0, 0%)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 0, -10%)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 0, 45.6%)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0.456);\
         \n}\n"
        );
    }
}
mod unitless {
    use super::runner;

    mod above {
        use super::runner;

        #[test]
        fn max() {
            assert_eq!(
                runner().ok("a {b: rgb(0, 0, 0, 250)}\n"),
                "a {\
         \n  b: rgb(0, 0, 0);\
         \n}\n"
            );
        }
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 0, 1)}\n"),
            "a {\
         \n  b: rgb(0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 0, 0)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 0, -10)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 0, 0.456)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0.456);\
         \n}\n"
        );
    }
}
