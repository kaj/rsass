//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/three_args/special_functions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("special_functions")
}

mod calc {
    #[allow(unused)]
    use super::runner;

    mod string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(string.unquote(\"calc(1)\"), 2, 3)}\n"),
                "a {\
         \n  b: rgb(calc(1), 2, 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1, string.unquote(\"calc(2)\"), 3)}\n"),
                "a {\
         \n  b: rgb(1, calc(2), 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1, 2, string.unquote(\"calc(3)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, calc(3));\
         \n}\n"
            );
        }
    }
}
mod clamp {
    #[allow(unused)]
    use super::runner;

    mod string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(string.unquote(\"clamp(1, 2, 3)\"), 2, 3)}\n"),
                "a {\
         \n  b: rgb(clamp(1, 2, 3), 2, 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1, string.unquote(\"clamp(2, 3, 4)\"), 3)}\n"),
                "a {\
         \n  b: rgb(1, clamp(2, 3, 4), 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1, 2, string.unquote(\"clamp(3, 4, 5)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, clamp(3, 4, 5));\
         \n}\n"
            );
        }
    }
}
mod env {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgb(env(--foo), 2, 3)}\n"),
            "a {\
         \n  b: rgb(env(--foo), 2, 3);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgb(1, env(--foo), 3)}\n"),
            "a {\
         \n  b: rgb(1, env(--foo), 3);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgb(1, 2, env(--foo))}\n"),
            "a {\
         \n  b: rgb(1, 2, env(--foo));\
         \n}\n"
        );
    }
}
mod max {
    #[allow(unused)]
    use super::runner;

    mod string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(string.unquote(\"max(1)\"), 2, 3)}\n"),
                "a {\
         \n  b: rgb(max(1), 2, 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1, string.unquote(\"max(2)\"), 3)}\n"),
                "a {\
         \n  b: rgb(1, max(2), 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1, 2, string.unquote(\"max(3)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, max(3));\
         \n}\n"
            );
        }
    }
}
mod min {
    #[allow(unused)]
    use super::runner;

    mod string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(string.unquote(\"min(1)\"), 2, 3)}\n"),
                "a {\
         \n  b: rgb(min(1), 2, 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1, string.unquote(\"min(2)\"), 3)}\n"),
                "a {\
         \n  b: rgb(1, min(2), 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1, 2, string.unquote(\"min(3)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, min(3));\
         \n}\n"
            );
        }
    }
}
mod var {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgb(var(--foo), 2, 3)}\n"),
            "a {\
         \n  b: rgb(var(--foo), 2, 3);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgb(1, var(--foo), 3)}\n"),
            "a {\
         \n  b: rgb(1, var(--foo), 3);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgb(1, 2, var(--foo))}\n"),
            "a {\
         \n  b: rgb(1, 2, var(--foo));\
         \n}\n"
        );
    }
}
