//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/hwb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

#[test]
#[ignore] // wrong result
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(#66cc99, $whiteness: -50%, $blackness: 50%)}\n"
        ),
        "a {\
         \n  b: rgb(51, 102, 76.5);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(#66cc99, $whiteness: -50%, $blackness: 50%, $alpha: -70%)}\n"
        ),
        "a {\
         \n  b: rgba(51, 102, 76.5, 0.3);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(rgba(#66cc99, 0.7), $whiteness: -50%, $blackness: 50%)}\n"
        ),
        "a {\
         \n  b: rgba(51, 102, 76.5, 0.7);\
         \n}\n"
    );
}
mod blackness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#33cc80, $blackness: 50%)}\n"),
            "a {\
         \n  b: rgb(51, 102, 76.6666666667);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#339966, $blackness: -50%)}\n"),
            "a {\
         \n  b: rgb(51, 204, 127.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#339966, $blackness: 100%)}\n"),
            "a {\
         \n  b: rgb(42.5, 42.5, 42.5);\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#339966, $blackness: -100%)}\n"),
            "a {\
         \n  b: #33ff99;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#339966, $blackness: 0%)}\n"),
            "a {\
         \n  b: #339966;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale($color: #66cc99, $whiteness: -50%, $blackness: 50%)}\n"
        ),
        "a {\
         \n  b: rgb(51, 102, 76.5);\
         \n}\n"
    );
}
mod whiteness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#33cc80, $whiteness: 50%)}\n"),
            "a {\
         \n  b: rgb(153, 204, 178.6666666667);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#66cc99, $whiteness: -50%)}\n"),
            "a {\
         \n  b: rgb(51, 204, 127.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#66cc99, $whiteness: 100%)}\n"),
            "a {\
         \n  b: rgb(212.5, 212.5, 212.5);\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#66cc99, $whiteness: -100%)}\n"),
            "a {\
         \n  b: #00cc66;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#66cc99, $whiteness: 0%)}\n"),
            "a {\
         \n  b: #66cc99;\
         \n}\n"
        );
    }
}
