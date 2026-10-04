//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/unusual_numbers.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unusual_numbers")
}

mod infinity {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn linear() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 0 0), $a: calc(infinity))};\n"),
            "a {\
         \n  b: lab(50% calc(infinity) 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn polar() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lch(50% 100% 0deg), $hue: calc(infinity))};\n"
        ),
        "a {\
         \n  b: lch(50% 150 0deg);\
         \n}\n"
    );
    }
}
mod nan {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn linear() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 0 0), $a: calc(nan))};\n"),
            "a {\
         \n  b: lab(50% 0 0);\
         \n}\n"
        );
    }
}
mod negative_infinity {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn linear() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 0 0), $a: calc(-infinity))};\n"),
            "a {\
         \n  b: lab(50% calc(-infinity) 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn polar() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lch(50% 100% 0deg), $hue: calc(-infinity))};\n"
        ),
        "a {\
         \n  b: lch(50% 150 0deg);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn negative_zero() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 0 0), $a: -0)};\n"),
        "a {\
         \n  b: lab(50% 0 0);\
         \n}\n"
    );
}
