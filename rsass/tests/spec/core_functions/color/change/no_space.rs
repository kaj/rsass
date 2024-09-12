//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/no_space.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_space")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(rgba(red, 0.5), $alpha: 0.72)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.72);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(rgba(red, 0.5), $alpha: 0.36)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.36);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(rgba(red, 0.5), $alpha: 1)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(rgba(red, 0.5), $alpha: 0)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_legacy() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 50 50), $alpha: 0.36)}\n"),
            "a {\
         \n  b: lab(50% 50 50 / 0.36);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $alpha: none)}\n"),
            "a {\
         \n  b: rgb(255 0 0 / none);\
         \n}\n"
        );
    }
    mod units {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn percent() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $alpha: 50%)}\n"),
                "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
         \n}\n"
            );
        }
        #[test]
        fn unitless() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $alpha: 0.5)}\n"),
                "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
         \n}\n"
            );
        }
        #[test]
        fn unknown() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $alpha: 0.5px)}\n"),
                "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
         \n}\n"
            );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.change($color: red)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn non_legacy() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 50 50))}\n"),
        "a {\
         \n  b: lab(50% 50 50);\
         \n}\n"
    );
}
#[test]
fn positional() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.change(red)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
