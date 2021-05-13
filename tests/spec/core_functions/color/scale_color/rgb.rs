//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color/rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "a {b: scale-color(sienna, $red: 12%, $green: 24%, $blue: 48%)}\n"
        ),
        "a {\
         \n  b: #ab7c92;\
         \n}\n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "a {b: scale-color(sienna, $red: 12%, $green: 24%, $blue: 48%, $alpha: -70%)}\n"
        ),
        "a {\
         \n  b: rgba(171, 124, 146, 0.3);\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "a {b: scale-color(rgba(sienna, 0.3), $red: 12%, $green: 24%, $blue: 48%)}\n"
        ),
        "a {\
         \n  b: rgba(171, 124, 146, 0.3);\
         \n}\n"
    );
}
mod blue {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: scale-color(salmon, $blue: 42%)}\n"),
            "a {\
         \n  b: #fa80ad;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: scale-color(slategray, $blue: -16%)}\n"),
            "a {\
         \n  b: #708079;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: scale-color(black, $blue: 100%)}\n"),
            "a {\
         \n  b: blue;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: scale-color(blue, $blue: -100%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: scale-color(black, $blue: 0%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
mod green {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: scale-color(cadetblue, $green: 12%)}\n"),
            "a {\
         \n  b: #5faaa0;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: scale-color(seagreen, $green: -86%)}\n"),
            "a {\
         \n  b: #2e1357;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: scale-color(black, $green: 100%)}\n"),
            "a {\
         \n  b: lime;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: scale-color(lime, $green: -100%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: scale-color(black, $green: 0%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: scale-color($color: sienna, $red: 12%, $green: 24%, $blue: 48%)}\n"
        ),
        "a {\
         \n  b: #ab7c92;\
         \n}\n"
    );
}
mod red {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: scale-color(turquoise, $red: 86%)}\n"),
            "a {\
         \n  b: #e4e0d0;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: scale-color(lightcoral, $red: -33%)}\n"),
            "a {\
         \n  b: #a18080;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: scale-color(black, $red: 100%)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: scale-color(red, $red: -100%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: scale-color(black, $red: 0%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
