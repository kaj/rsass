//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/one_arg/special_functions/no_alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_alpha")
}

mod attr {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgb(attr(c, %) 2 3)}\n"),
            "a {\
         \n  b: rgb(attr(c, %), 2, 3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgb(1 attr(c, %) 3)}\n"),
            "a {\
         \n  b: rgb(1, attr(c, %), 3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgb(1 2 attr(c, %))}\n"),
            "a {\
         \n  b: rgb(1, 2, attr(c, %));\
         \n}\n"
        );
    }
}
mod calc {
    use super::runner;

    mod calculation {
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: rgb(calc(1px + 1%) 2 3)}\n"),
                "a {\
         \n  b: rgb(calc(1px + 1%), 2, 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: rgb(1 calc(1px + 1%) 3)}\n"),
                "a {\
         \n  b: rgb(1, calc(1px + 1%), 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: rgb(1 2 calc(1px + 1%))}\n"),
                "a {\
         \n  b: rgb(1, 2, calc(1px + 1%));\
         \n}\n"
            );
        }
    }
    mod string {
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(string.unquote(\"calc(1)\") 2 3)}\n"),
                "a {\
         \n  b: rgb(calc(1), 2, 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1 string.unquote(\"calc(2)\") 3)}\n"),
                "a {\
         \n  b: rgb(1, calc(2), 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1 2 string.unquote(\"calc(3)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, calc(3));\
         \n}\n"
            );
        }
    }
}
mod clamp {
    use super::runner;

    mod string {
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(string.unquote(\"clamp(1, 2, 3)\") 2 3)}\n"),
                "a {\
         \n  b: rgb(clamp(1, 2, 3), 2, 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1 string.unquote(\"clamp(2, 3, 4)\") 3)}\n"),
                "a {\
         \n  b: rgb(1, clamp(2, 3, 4), 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1 2 string.unquote(\"clamp(3, 4, 5)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, clamp(3, 4, 5));\
         \n}\n"
            );
        }
    }
}
mod env {
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgb(env(--foo) 2 3)}\n"),
            "a {\
         \n  b: rgb(env(--foo), 2, 3);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgb(1 env(--foo) 3)}\n"),
            "a {\
         \n  b: rgb(1, env(--foo), 3);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgb(1 2 env(--foo))}\n"),
            "a {\
         \n  b: rgb(1, 2, env(--foo));\
         \n}\n"
        );
    }
}
mod max {
    use super::runner;

    mod string {
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(string.unquote(\"max(1)\") 2 3)}\n"),
                "a {\
         \n  b: rgb(max(1), 2, 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1 string.unquote(\"max(2)\") 3)}\n"),
                "a {\
         \n  b: rgb(1, max(2), 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1 2 string.unquote(\"max(3)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, max(3));\
         \n}\n"
            );
        }
    }
}
mod min {
    use super::runner;

    mod string {
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(string.unquote(\"min(1)\") 2 3)}\n"),
                "a {\
         \n  b: rgb(min(1), 2, 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1 string.unquote(\"min(2)\") 3)}\n"),
                "a {\
         \n  b: rgb(1, min(2), 3);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: rgb(1 2 string.unquote(\"min(3)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, min(3));\
         \n}\n"
            );
        }
    }
}
mod multi_argument_var {
    use super::runner;

    #[test]
    fn t1_of_1() {
        assert_eq!(
            runner().ok("a {b: rgb(var(--foo))}\n"),
            "a {\
         \n  b: rgb(var(--foo));\
         \n}\n"
        );
    }
    #[test]
    fn t1_of_2() {
        assert_eq!(
        runner().ok(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
             \na {b: rgb(var(--foo) 2)}\n"
        ),
        "a {\
         \n  b: rgb(var(--foo) 2);\
         \n}\n"
    );
    }
    #[test]
    fn t2_of_2() {
        assert_eq!(
            runner().ok("a {b: rgb(1 var(--foo))}\n"),
            "a {\
         \n  b: rgb(1 var(--foo));\
         \n}\n"
        );
    }
}
mod var {
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgb(var(--foo) 2 3)}\n"),
            "a {\
         \n  b: rgb(var(--foo), 2, 3);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgb(1 var(--foo) 3)}\n"),
            "a {\
         \n  b: rgb(1, var(--foo), 3);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgb(1 2 var(--foo))}\n"),
            "a {\
         \n  b: rgb(1, 2, var(--foo));\
         \n}\n"
        );
    }
}
