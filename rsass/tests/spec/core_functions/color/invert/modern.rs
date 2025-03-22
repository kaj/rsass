//! Tests auto-converted from "sass-spec/spec/core_functions/color/invert/modern.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("modern")
}

#[test]
#[ignore] // unexepected error
fn negative_min() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.invert(lab(20% -30 110), $space: lab)}\n"),
        "a {\
         \n  b: lab(80% 30 -110);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn polar() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.invert(lch(20% 80 50deg), $space: lch)}\n"),
        "a {\
         \n  b: lch(80% 80 230deg);\
         \n}\n"
    );
}
mod space {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn case() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.invert(lch(20% 80 50deg), $space: DISPLAY-p3)}\n"),
            "a {\
         \n  b: lch(97.4036876937% 47.4823953357 211.8732507605deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn legacy() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.invert(lch(20% 80 50deg), $space: rgb)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz 0.8040115061 1.0427791446 1.4858460828) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn missing_converted() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.invert(color(rec2020 none none none), $space: lab)}\n"
        ),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn modern() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.invert(lch(20% 80 50deg), $space: xyz)}\n"),
            "a {\
         \n  b: lch(98.9503159926% 3.6006542179 45.9089280707deg);\
         \n}\n"
        );
    }
    mod powerless {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(lch(40% 0% 120deg), $space: lab)}\n"),
                "a {\
         \n  b: lch(60% 0 none);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.invert(lch(40% 0% 120deg), $space: lch)}\n"),
                "a {\
         \n  b: lch(60% 0 300deg);\
         \n}\n"
            );
        }
    }
}
mod weight {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.invert(color(a98-rgb 0.1 0.4 0.8), 60%, $space: a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.58 0.52 0.44);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.invert(color(a98-rgb 0.1 0.4 0.8), 100%, $space: a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.9 0.6 0.2);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn middle() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.invert(color(a98-rgb 0.1 0.4 0.8), 50%, $space: a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.5 0.5 0.5);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.invert(color(a98-rgb 0.1 0.4 0.8), 0%, $space: a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.1 0.4 0.8);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn zero_min() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.invert(color(rec2020 0 0.4 1), $space: rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 1 0.6 0);\
         \n}\n"
    );
}
