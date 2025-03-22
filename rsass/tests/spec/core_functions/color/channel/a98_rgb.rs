//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/a98-rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("a98-rgb")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"blue\", $space: a98-rgb)}\n"),
            "a {\
         \n  b: 0.7893042668;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"green\", $space: a98-rgb)}\n"),
            "a {\
         \n  b: 0.7473920857;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"red\", $space: a98-rgb)}\n"),
            "a {\
         \n  b: 0.9363244101;\
         \n}\n"
        );
    }
}
mod local {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(color(a98-rgb 0.2 0.5 0.8), \"blue\")}\n"),
            "a {\
         \n  b: 0.8;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.channel(color(a98-rgb 0.2 0.5 0.8), \"green\")}\n"
            ),
            "a {\
         \n  b: 0.5;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(color(a98-rgb 0.2 0.5 0.8), \"red\")}\n"),
            "a {\
         \n  b: 0.2;\
         \n}\n"
        );
    }
}
