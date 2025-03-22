//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/rec2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"blue\", $space: rec2020)}\n"),
            "a {\
         \n  b: 0.7726929727;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"green\", $space: rec2020)}\n"),
            "a {\
         \n  b: 0.747938727;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"red\", $space: rec2020)}\n"),
            "a {\
         \n  b: 0.9098509852;\
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
             \na {b: color.channel(color(rec2020 0.2 0.5 0.8), \"blue\")}\n"),
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
             \na {b: color.channel(color(rec2020 0.2 0.5 0.8), \"green\")}\n"
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
             \na {b: color.channel(color(rec2020 0.2 0.5 0.8), \"red\")}\n"),
            "a {\
         \n  b: 0.2;\
         \n}\n"
        );
    }
}
