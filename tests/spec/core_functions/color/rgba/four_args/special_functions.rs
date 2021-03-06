//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/four_args/special_functions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod calc {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgba(calc(1), 2, 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(calc(1), 2, 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgba(1, calc(2), 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(1, calc(2), 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, calc(3), 0.4)}\n"),
            "a {\
         \n  b: rgba(1, 2, calc(3), 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_4() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, 3, calc(0.4))}\n"),
            "a {\
         \n  b: rgba(1, 2, 3, calc(0.4));\
         \n}\n"
        );
    }
}
mod clamp {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgba(clamp(1, 2, 3), 2, 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(clamp(1, 2, 3), 2, 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgba(1, clamp(2, 3, 4), 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(1, clamp(2, 3, 4), 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, clamp(3, 4, 5), 0.4)}\n"),
            "a {\
         \n  b: rgba(1, 2, clamp(3, 4, 5), 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_4() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, 3, clamp(0.4, 0.5, 0.6))}\n"),
            "a {\
         \n  b: rgba(1, 2, 3, clamp(0.4, 0.5, 0.6));\
         \n}\n"
        );
    }
}
mod env {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgba(env(--foo), 2, 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(env(--foo), 2, 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgba(1, env(--foo), 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(1, env(--foo), 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, env(--foo), 0.4)}\n"),
            "a {\
         \n  b: rgba(1, 2, env(--foo), 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_4() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, 3, env(--foo))}\n"),
            "a {\
         \n  b: rgba(1, 2, 3, env(--foo));\
         \n}\n"
        );
    }
}
mod max {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgba(max(1), 2, 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(max(1), 2, 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgba(1, max(2), 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(1, max(2), 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, max(3), 0.4)}\n"),
            "a {\
         \n  b: rgba(1, 2, max(3), 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_4() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, 3, max(0.4))}\n"),
            "a {\
         \n  b: rgba(1, 2, 3, max(0.4));\
         \n}\n"
        );
    }
}
mod min {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgba(min(1), 2, 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(min(1), 2, 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgba(1, min(2), 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(1, min(2), 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, min(3), 0.4)}\n"),
            "a {\
         \n  b: rgba(1, 2, min(3), 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_4() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, 3, min(0.4))}\n"),
            "a {\
         \n  b: rgba(1, 2, 3, min(0.4));\
         \n}\n"
        );
    }
}
mod var {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgba(var(--foo), 2, 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(var(--foo), 2, 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgba(1, var(--foo), 3, 0.4)}\n"),
            "a {\
         \n  b: rgba(1, var(--foo), 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, var(--foo), 0.4)}\n"),
            "a {\
         \n  b: rgba(1, 2, var(--foo), 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_4() {
        assert_eq!(
            runner().ok("a {b: rgba(1, 2, 3, var(--foo))}\n"),
            "a {\
         \n  b: rgba(1, 2, 3, var(--foo));\
         \n}\n"
        );
    }
}
