//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/display_p3_linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3_linear")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.channel(pink, \"blue\", $space: display-p3-linear)}\n"
        ),
        "a {\
         \n  b: 0.5990085465;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.channel(pink, \"green\", $space: display-p3-linear)}\n"
        ),
        "a {\
         \n  b: 0.5428121603;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.channel(pink, \"red\", $space: display-p3-linear)}\n"
        ),
        "a {\
         \n  b: 0.9160449504;\
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
             \na {b: color.channel(color(display-p3-linear 0.2 0.5 0.8), \"blue\")}\n"
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
             \na {b: color.channel(color(display-p3-linear 0.2 0.5 0.8), \"green\")}\n"
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
             \na {b: color.channel(color(display-p3-linear 0.2 0.5 0.8), \"red\")}\n"
        ),
        "a {\
         \n  b: 0.2;\
         \n}\n"
    );
    }
}
