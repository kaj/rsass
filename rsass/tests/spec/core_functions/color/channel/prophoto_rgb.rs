//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/prophoto-rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("prophoto-rgb")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"blue\", $space: prophoto-rgb)}\n"),
            "a {\
         \n  b: 0.7499576055;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.channel(pink, \"green\", $space: prophoto-rgb)}\n"
            ),
            "a {\
         \n  b: 0.7357592889;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"red\", $space: prophoto-rgb)}\n"),
            "a {\
         \n  b: 0.8755603733;\
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
             \na {b: color.channel(color(prophoto-rgb 0.2 0.5 0.8), \"blue\")}\n"
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
             \na {b: color.channel(color(prophoto-rgb 0.2 0.5 0.8), \"green\")}\n"
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
             \na {b: color.channel(color(prophoto-rgb 0.2 0.5 0.8), \"red\")}\n"
        ),
        "a {\
         \n  b: 0.2;\
         \n}\n"
    );
    }
}
