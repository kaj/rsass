//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color/hwb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "a {b: scale-color(#66cc99, $whiteness: -50%, $blackness: 50%)}\n"
        ),
        "a {\
         \n  b: #33664d;\
         \n}\n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "a {b: scale-color(#66cc99, $whiteness: -50%, $blackness: 50%, $alpha: -70%)}\n"
        ),
        "a {\
         \n  b: rgba(51, 102, 77, 0.3);\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "a {b: scale-color(rgba(#66cc99, 0.7), $whiteness: -50%, $blackness: 50%)}\n"
        ),
        "a {\
         \n  b: rgba(51, 102, 77, 0.7);\
         \n}\n"
    );
}
mod blackness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: scale-color(#33cc80, $blackness: 50%)}\n"),
            "a {\
         \n  b: #33664d;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: scale-color(#339966, $blackness: -50%)}\n"),
            "a {\
         \n  b: #33cc80;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: scale-color(#339966, $blackness: 100%)}\n"),
            "a {\
         \n  b: #2b2b2b;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: scale-color(#339966, $blackness: -100%)}\n"),
            "a {\
         \n  b: #33ff99;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: scale-color(#339966, $blackness: 0%)}\n"),
            "a {\
         \n  b: #339966;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: scale-color($color: #66cc99, $whiteness: -50%, $blackness: 50%)}\n"
        ),
        "a {\
         \n  b: #33664d;\
         \n}\n"
    );
}
mod whiteness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: scale-color(#33cc80, $whiteness: 50%)}\n"),
            "a {\
         \n  b: #99ccb3;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: scale-color(#66cc99, $whiteness: -50%)}\n"),
            "a {\
         \n  b: #33cc80;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: scale-color(#66cc99, $whiteness: 100%)}\n"),
            "a {\
         \n  b: #d5d5d5;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: scale-color(#66cc99, $whiteness: -100%)}\n"),
            "a {\
         \n  b: #00cc66;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: scale-color(#66cc99, $whiteness: 0%)}\n"),
            "a {\
         \n  b: #66cc99;\
         \n}\n"
        );
    }
}
