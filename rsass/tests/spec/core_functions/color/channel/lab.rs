//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/lab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"a\", $space: lab)}\n"),
            "a {\
         \n  b: 24.4367702881;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"b\", $space: lab)}\n"),
            "a {\
         \n  b: 3.7594237161;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"lightness\", $space: lab)}\n"),
            "a {\
         \n  b: 83.7872528656%;\
         \n}\n"
        );
    }
}
mod local {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(lab(50% -150 30), \"a\")}\n"),
            "a {\
         \n  b: -150;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(lab(50% -150 30), \"b\")}\n"),
            "a {\
         \n  b: 30;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(lab(50 -150 30), \"lightness\")}\n"),
            "a {\
         \n  b: 50%;\
         \n}\n"
        );
    }
}
