//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

mod clip {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-gamut(#abcdef, $method: clip)}\n"),
            "a {\
         \n  b: #abcdef;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color.change(#abcdef, $red: 300), $method: clip)}\n"
        ),
        "a {\
         \n  b: #ffcdef;\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-gamut(#abcdef, $method: local-minde)}\n"),
            "a {\
         \n  b: #abcdef;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color.change(#abcdef, $red: 300), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: rgb(255, 222.6637650714, 237.9231048356);\
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
             \n    color.change(black, $red: -5, $green: -5),\
             \n    $method: local-minde\
             \n  );\
             \n}\n"),
            "a {\
         \n  b: black;\
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
             \n    color.change(#e6ff00, $green: 280, $blue: -25),\
             \n    $method: local-minde\
             \n  );\
             \n}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
}
