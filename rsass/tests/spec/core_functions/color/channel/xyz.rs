//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/xyz.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn x() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"x\", $space: xyz)}\n"),
            "a {\
         \n  b: 0.7086623629;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn y() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"y\", $space: xyz)}\n"),
            "a {\
         \n  b: 0.6327286137;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn z() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"z\", $space: xyz)}\n"),
            "a {\
         \n  b: 0.6498196913;\
         \n}\n"
        );
    }
}
mod local {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn x() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(color(xyz-d65 -0.2 0.5 1.8), \"x\")}\n"),
            "a {\
         \n  b: -0.2;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn y() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(color(xyz-d65 -0.2 0.5 1.8), \"y\")}\n"),
            "a {\
         \n  b: 0.5;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn z() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(color(xyz-d65 -0.2 0.5 1.8), \"z\")}\n"),
            "a {\
         \n  b: 1.8;\
         \n}\n"
        );
    }
}
