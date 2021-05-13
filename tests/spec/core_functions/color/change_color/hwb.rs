//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/hwb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "a {b: change-color(blue, $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
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
            "a {b: change-color(blue, $hue: 150, $whiteness: 20%, $blackness: 40%, $alpha: 0.3)}\n"
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
            "a {b: change-color(rgba(blue, 0.7), $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
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
            runner().ok("a {b: change-color(#993333, $blackness: 80%)}\n"),
            "a {\
         \n  b: #333333;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: change-color(#993333, $blackness: 20%)}\n"),
            "a {\
         \n  b: #cc3333;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: change-color(#993333, $blackness: 100%)}\n"),
            "a {\
         \n  b: #2b2b2b;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: change-color(#993333, $blackness: 0%)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: change-color($color: blue, $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
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
            runner().ok("a {b: change-color(#cc6666, $whiteness: 80%)}\n"),
            "a {\
         \n  b: #cccccc;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: change-color(#cc6666, $whiteness: 20%)}\n"),
            "a {\
         \n  b: #cc3333;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: change-color(#cc6666, $whiteness: 100%)}\n"),
            "a {\
         \n  b: #d5d5d5;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: change-color(#cc6666, $whiteness: 0%)}\n"),
            "a {\
         \n  b: #cc0000;\
         \n}\n"
        );
    }
}
