//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/hwb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(blue, $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
        ),
        "a {\
         \n  b: #339966;\
         \n}\n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(blue, $hue: 150, $whiteness: 20%, $blackness: 40%, $alpha: 0.3)}\n"
        ),
        "a {\
         \n  b: rgba(51, 153, 102, 0.3);\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(rgba(blue, 0.7), $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
        ),
        "a {\
         \n  b: rgba(51, 153, 102, 0.7);\
         \n}\n"
    );
}
mod blackness {
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(#993333, $blackness: 80%)}\n"),
            "a {\
         \n  b: #333333;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(#993333, $blackness: 20%)}\n"),
            "a {\
         \n  b: #cc3333;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(#993333, $blackness: 100%)}\n"),
            "a {\
         \n  b: rgb(42.5, 42.5, 42.5);\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(#993333, $blackness: 0%)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(hwb(180deg 30% 50%), $blackness: none)}\n"),
            "a {\
         \n  b: hwb(180deg 30% none);\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change($color: blue, $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
        ),
        "a {\
         \n  b: #339966;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(#993333, $whiteness: 50%, $blackness: -20%)}\n"
        ),
        "a {\
         \n  b: hsl(0, 233.3333333333%, 85%);\
         \n}\n"
    );
}
mod whiteness {
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(#cc6666, $whiteness: 80%)}\n"),
            "a {\
         \n  b: #cccccc;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(#cc6666, $whiteness: 20%)}\n"),
            "a {\
         \n  b: #cc3333;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(#cc6666, $whiteness: 100%)}\n"),
            "a {\
         \n  b: rgb(212.5, 212.5, 212.5);\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(#cc6666, $whiteness: 0%)}\n"),
            "a {\
         \n  b: #cc0000;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(hwb(180deg 30% 50%), $whiteness: none)}\n"),
            "a {\
         \n  b: hwb(180deg none 50%);\
         \n}\n"
        );
    }
}
