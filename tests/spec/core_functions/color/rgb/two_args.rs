//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/two_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod clamped {
    #[allow(unused)]
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
    #[allow(unused)]
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
    #[allow(unused)]
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
mod transparent_to {
    #[allow(unused)]
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
