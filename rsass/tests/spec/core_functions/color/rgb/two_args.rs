//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/two_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("two_args")
}

mod clamped {
    use super::runner;

    #[test]
    fn opaque() {
        assert_eq!(
            runner().ok("a {b: rgb(#123, 1.1)}\n"),
            "a {\
         \n  b: #112233;\
         \n}\n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            runner().ok("a {b: rgb(#123, -0.1)}\n"),
            "a {\
         \n  b: rgba(17, 34, 51, 0);\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: rgb($color: #123, $alpha: 0.5)}\n"),
        "a {\
         \n  b: rgba(17, 34, 51, 0.5);\
         \n}\n"
    );
}
mod opaque_to {
    use super::runner;

    #[test]
    fn opaque() {
        assert_eq!(
            runner().ok("a {b: rgb(#123, 1)}\n"),
            "a {\
         \n  b: #112233;\
         \n}\n"
        );
    }
    #[test]
    fn partial() {
        assert_eq!(
            runner().ok("a {b: rgb(#123, 0.5)}\n"),
            "a {\
         \n  b: rgba(17, 34, 51, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            runner().ok("a {b: rgb(#123, 0)}\n"),
            "a {\
         \n  b: rgba(17, 34, 51, 0);\
         \n}\n"
        );
    }
}
mod partial_to {
    use super::runner;

    #[test]
    fn opaque() {
        assert_eq!(
            runner().ok("a {b: rgb(rgba(0, 0, 255, 0.3), 1)}\n"),
            "a {\
         \n  b: blue;\
         \n}\n"
        );
    }
    #[test]
    fn partial() {
        assert_eq!(
            runner().ok("a {b: rgb(rgba(0, 0, 255, 0.3), 0.5)}\n"),
            "a {\
         \n  b: rgba(0, 0, 255, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            runner().ok("a {b: rgb(rgba(0, 0, 255, 0.3), 0)}\n"),
            "a {\
         \n  b: rgba(0, 0, 255, 0);\
         \n}\n"
        );
    }
}
mod special_functions {
    use super::runner;

    mod calc {
        use super::runner;

        mod string {
            use super::runner;

            #[test]
            fn arg_2() {
                assert_eq!(
                    runner().ok("@use \"sass:string\";\
             \na {b: rgb(blue, string.unquote(\"calc(0.4)\"))}\n"),
                    "a {\
         \n  b: rgb(0, 0, 255, calc(0.4));\
         \n}\n"
                );
            }
        }
    }
    mod var {
        use super::runner;

        mod args {
            use super::runner;

            #[test]
            fn alpha() {
                assert_eq!(
                    runner().ok("a {b: rgb(blue, var(--foo))}\n"),
                    "a {\
         \n  b: rgb(0, 0, 255, var(--foo));\
         \n}\n"
                );
            }
            #[test]
            fn both() {
                assert_eq!(
                    runner().ok("a {b: rgb(var(--foo), var(--foo))}\n"),
                    "a {\
         \n  b: rgb(var(--foo), var(--foo));\
         \n}\n"
                );
            }
            #[test]
            fn color() {
                assert_eq!(
                    runner().ok("a {b: rgb(var(--foo), 0.4)}\n"),
                    "a {\
         \n  b: rgb(var(--foo), 0.4);\
         \n}\n"
                );
            }
        }
    }
}
mod transparent_to {
    use super::runner;

    #[test]
    fn opaque() {
        assert_eq!(
            runner().ok("a {b: rgb(transparent, 1)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn partial() {
        assert_eq!(
            runner().ok("a {b: rgb(transparent, 0.5)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            runner().ok("a {b: rgb(transparent, 0)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
}
