//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/xyz-d50.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz-d50")
}

mod foreign {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn x() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"x\", $space: xyz-d50)}\n"),
            "a {\
         \n  b: 0.7245316215;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn y() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"y\", $space: xyz-d50)}\n"),
            "a {\
         \n  b: 0.636577424;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn z() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"z\", $space: xyz-d50)}\n"),
            "a {\
         \n  b: 0.4915583688;\
         \n}\n"
        );
    }
}
mod local {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn x() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(color(xyz-d50 -0.2 0.5 1.8), \"x\")}\n"),
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
             \na {b: color.channel(color(xyz-d50 -0.2 0.5 1.8), \"y\")}\n"),
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
             \na {b: color.channel(color(xyz-d50 -0.2 0.5 1.8), \"z\")}\n"),
            "a {\
         \n  b: 1.8;\
         \n}\n"
        );
    }
}
