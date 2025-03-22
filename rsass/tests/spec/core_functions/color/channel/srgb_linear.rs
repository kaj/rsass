//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/srgb-linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb-linear")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"blue\", $space: srgb-linear)}\n"),
            "a {\
         \n  b: 0.5972017884;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"green\", $space: srgb-linear)}\n"),
            "a {\
         \n  b: 0.5271151257;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"red\", $space: srgb-linear)}\n"),
            "a {\
         \n  b: 1;\
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.channel(color(srgb-linear 0.2 0.5 0.8), \"blue\")}\n"
        ),
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
             \na {b: color.channel(color(srgb-linear 0.2 0.5 0.8), \"green\")}\n"
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.channel(color(srgb-linear 0.2 0.5 0.8), \"red\")}\n"
        ),
        "a {\
         \n  b: 0.2;\
         \n}\n"
    );
    }
}
