//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/hwb.hrx"

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
         \n  b: rgb(20%, 40%, 30%);\
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
         \n  b: rgba(20%, 40%, 30%, 0.3);\
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
         \n  b: rgba(20%, 40%, 30%, 0.7);\
         \n}\n"
    );
}
mod blackness {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#33cc80, $blackness: 50%)}\n"),
            "a {\
         \n  b: rgb(20%, 40%, 30.0653594771%);\
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
         \n  b: rgb(20%, 80%, 50%);\
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
         \n  b: rgb(16.6666666667%, 16.6666666667%, 16.6666666667%);\
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
         \n  b: rgb(20%, 40%, 30%);\
         \n}\n"
    );
}
mod whiteness {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#33cc80, $whiteness: 50%)}\n"),
            "a {\
         \n  b: rgb(60%, 80%, 70.0653594771%);\
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
         \n  b: rgb(20%, 80%, 50%);\
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
         \n  b: rgb(83.3333333333%, 83.3333333333%, 83.3333333333%);\
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
