//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/hwb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

mod clip {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-gamut(hwb(70deg 30% 30%), $method: clip)}\n"),
            "a {\
         \n  b: hsl(70, 40%, 50%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-gamut(hwb(70deg 200% -50%), $method: clip)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
}
mod local_minde {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(hwb(70deg 30% 30%), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: hsl(70, 40%, 50%);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(hwb(70deg -5% -5%), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: hsl(75.715745279, 100%, 75.8982967878%);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_black() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(hwb(70deg -2% 100%), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: hwb(none 0% 100%);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_white() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(hwb(70deg -10% -10%), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: hwb(none 100% 0%);\
         \n}\n"
    );
    }
}
