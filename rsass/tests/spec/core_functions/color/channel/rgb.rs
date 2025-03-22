//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"blue\")}\n"),
            "a {\
         \n  b: 203;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"green\")}\n"),
            "a {\
         \n  b: 192;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"red\")}\n"),
            "a {\
         \n  b: 255;\
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
             \na {b: color.channel(#abcdef, \"blue\")}\n"),
            "a {\
         \n  b: 239;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(#abcdef, \"green\")}\n"),
            "a {\
         \n  b: 205;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(#abcdef, \"red\")}\n"),
            "a {\
         \n  b: 171;\
         \n}\n"
        );
    }
}
