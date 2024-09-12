//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/display-p3.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display-p3")
}

mod foreign {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"blue\", $space: display-p3)}\n"),
            "a {\
         \n  b: 0.7971503319;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"green\", $space: display-p3)}\n"),
            "a {\
         \n  b: 0.7628803605;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"red\", $space: display-p3)}\n"),
            "a {\
         \n  b: 0.9621487118;\
         \n}\n"
        );
    }
}
mod local {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.channel(color(display-p3 0.2 0.5 0.8), \"blue\")}\n"
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
             \na {b: color.channel(color(display-p3 0.2 0.5 0.8), \"green\")}\n"
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
             \na {b: color.channel(color(display-p3 0.2 0.5 0.8), \"red\")}\n"
        ),
        "a {\
         \n  b: 0.2;\
         \n}\n"
    );
    }
}
