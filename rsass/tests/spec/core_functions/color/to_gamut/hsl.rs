//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

mod clip {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-gamut(hsl(70deg 50% 50%), $method: clip)}\n"),
            "a {\
         \n  b: hsl(70, 50%, 50%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(hsl(70deg 50% 50%), $saturation: 200%),\
             \n    $method: clip\
             \n  );\
             \n}\n"),
            "a {\
         \n  b: hsl(70, 100%, 50%);\
         \n}\n"
        );
    }
}
mod local_minde {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(hsl(70deg 50% 50%), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: hsl(70, 50%, 50%);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(hsl(70deg 50% 50%), $saturation: 107%),\
             \n    $method: local-minde\
             \n  );\
             \n}\n"),
            "a {\
         \n  b: hsl(73.5850769516, 100%, 67.4750278515%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_black() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(hsl(70deg 100% 50%), $lightness: -1%),\
             \n    $method: local-minde\
             \n  );\
             \n}\n"),
            "a {\
         \n  b: hsl(none 0% 0%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_white() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(hsl(70deg 50% 50%), $saturation: 120%),\
             \n    $method: local-minde\
             \n  );\
             \n}\n"),
            "a {\
         \n  b: hsl(none 0% 100%);\
         \n}\n"
        );
    }
}
