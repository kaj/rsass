//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_color/hwb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "a {b: adjust-color(red, $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
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
            "a {b: adjust-color(red, $hue: 150, $whiteness: 20%, $blackness: 40%, $alpha: -0.7)}\n"
        ),
        "a {\
         \n  b: rgba(51, 153, 102, 0.3);\
         \n}\n"
    );
}
#[test]
fn alpha_arg_above_max() {
    assert_eq!(
        runner().ok(
            "a {b: adjust-color(red, $hue: 150, $whiteness: 20%, $blackness: 40%, $alpha: 0.7)}\n"
        ),
        "a {\
         \n  b: #339966;\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "a {b: adjust-color(rgba(red, 0.7), $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
        ),
        "a {\
         \n  b: rgba(51, 153, 102, 0.7);\
         \n}\n"
    );
}
mod blackness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#993333, $blackness: 40%)}\n"),
            "a {\
         \n  b: #333333;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#993333, $blackness: -20%)}\n"),
            "a {\
         \n  b: #cc3333;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#993333, $blackness: 100%)}\n"),
            "a {\
         \n  b: #2b2b2b;\
         \n}\n"
        );
    }
    #[test]
    fn max_remaining() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#993333, $blackness: 60%)}\n"),
            "a {\
         \n  b: #2b2b2b;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#993333, $blackness: -100%)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
    #[test]
    fn min_remaining() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#993333, $blackness: -40%)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#993333, $blackness: 0%)}\n"),
            "a {\
         \n  b: #993333;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: adjust-color($color: red, $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
        ),
        "a {\
         \n  b: #339966;\
         \n}\n"
    );
}
mod whiteness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#cc6666, $whiteness: 40%)}\n"),
            "a {\
         \n  b: #cccccc;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#cc6666, $whiteness: -20%)}\n"),
            "a {\
         \n  b: #cc3333;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#cc6666, $whiteness: 100%)}\n"),
            "a {\
         \n  b: #d5d5d5;\
         \n}\n"
        );
    }
    #[test]
    fn max_remaining() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#cc6666, $whiteness: 60%)}\n"),
            "a {\
         \n  b: #d5d5d5;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#cc6666, $whiteness: -100%)}\n"),
            "a {\
         \n  b: #cc0000;\
         \n}\n"
        );
    }
    #[test]
    fn min_remaining() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#cc6666, $whiteness: -40%)}\n"),
            "a {\
         \n  b: #cc0000;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: adjust-color(#cc6666, $whiteness: 0%)}\n"),
            "a {\
         \n  b: #cc6666;\
         \n}\n"
        );
    }
}
