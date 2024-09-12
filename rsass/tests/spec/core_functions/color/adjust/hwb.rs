//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/hwb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
        ),
        "a {\
         \n  b: #339966;\
         \n}\n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.adjust(\
             \n    red,\
             \n    $hue: 150,\
             \n    $whiteness: 20%,\
             \n    $blackness: 40%,\
             \n    $alpha: -0.7\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: rgba(51, 153, 102, 0.3);\
         \n}\n"
    );
}
#[test]
fn alpha_arg_above_max() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.adjust(\
             \n    red,\
             \n    $hue: 150,\
             \n    $whiteness: 20%,\
             \n    $blackness: 40%,\
             \n    $alpha: 0.7\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: #339966;\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(rgba(red, 0.7), $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#993333, $blackness: 40%)}\n"),
            "a {\
         \n  b: #333333;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#993333, $blackness: -20%)}\n"),
            "a {\
         \n  b: #cc3333;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#993333, $blackness: 100%)}\n"),
            "a {\
         \n  b: rgb(31.875, 31.875, 31.875);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#993333, $blackness: 60%)}\n"),
            "a {\
         \n  b: rgb(42.5, 42.5, 42.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#993333, $blackness: -100%)}\n"),
            "a {\
         \n  b: hsl(0, 700%, 90%);\
         \n}\n"
        );
    }
    #[test]
    fn min_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#993333, $blackness: -40%)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#993333, $blackness: 0%)}\n"),
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
            "@use \"sass:color\";\
             \na {b: color.adjust($color: red, $hue: 150, $whiteness: 20%, $blackness: 40%)}\n"
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#cc6666, $whiteness: 40%)}\n"),
            "a {\
         \n  b: #cccccc;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#cc6666, $whiteness: -20%)}\n"),
            "a {\
         \n  b: #cc3333;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#cc6666, $whiteness: 100%)}\n"),
            "a {\
         \n  b: rgb(223.125, 223.125, 223.125);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#cc6666, $whiteness: 60%)}\n"),
            "a {\
         \n  b: rgb(212.5, 212.5, 212.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#cc6666, $whiteness: -100%)}\n"),
            "a {\
         \n  b: hsl(0, 700%, 10%);\
         \n}\n"
        );
    }
    #[test]
    fn min_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#cc6666, $whiteness: -40%)}\n"),
            "a {\
         \n  b: #cc0000;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#cc6666, $whiteness: 0%)}\n"),
            "a {\
         \n  b: #cc6666;\
         \n}\n"
        );
    }
}
