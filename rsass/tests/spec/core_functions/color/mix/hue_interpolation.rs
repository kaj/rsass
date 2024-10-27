//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/hue_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hue_interpolation")
}

#[test]
#[ignore] // unexepected error
fn case_insensitive() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(oklch(0.5 0.1 30), oklch(0.5 0.1 190), $method: oKlCh LONger HUE)}\n"
        ),
        "a {\
         \n  b: oklch(50% 0.1 290deg);\
         \n}\n"
    );
}
mod decreasing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn acute() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(oklch(0.5 0.1 30), oklch(0.5 0.1 190), $method: oklch decreasing hue)}\n"
        ),
        "a {\
         \n  b: oklch(50% 0.1 290deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn obtuse() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(oklch(0.5 0.1 30), oklch(0.5 0.1 230), $method: oklch decreasing hue)}\n"
        ),
        "a {\
         \n  b: oklch(50% 0.1 310deg);\
         \n}\n"
    );
    }
}
mod default {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn acute() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(oklch(0.5 0.1 30), oklch(0.5 0.1 190), $method: oklch)}\n"
        ),
        "a {\
         \n  b: oklch(50% 0.1 110deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn obtuse() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(oklch(0.5 0.1 30), oklch(0.5 0.1 230), $method: oklch)}\n"
        ),
        "a {\
         \n  b: oklch(50% 0.1 310deg);\
         \n}\n"
    );
    }
}
mod increasing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn acute() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(oklch(0.5 0.1 30), oklch(0.5 0.1 190), $method: oklch increasing hue)}\n"
        ),
        "a {\
         \n  b: oklch(50% 0.1 110deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn obtuse() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(oklch(0.5 0.1 30), oklch(0.5 0.1 230), $method: oklch increasing hue)}\n"
        ),
        "a {\
         \n  b: oklch(50% 0.1 130deg);\
         \n}\n"
    );
    }
}
mod longer {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn obtuse() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(oklch(0.5 0.1 30), oklch(0.5 0.1 230), $method: oklch longer hue)}\n"
        ),
        "a {\
         \n  b: oklch(50% 0.1 130deg);\
         \n}\n"
    );
    }
}
mod shorter {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn acute() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(oklch(0.5 0.1 30), oklch(0.5 0.1 190), $method: oklch shorter hue)}\n"
        ),
        "a {\
         \n  b: oklch(50% 0.1 110deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn obtuse() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(oklch(0.5 0.1 30), oklch(0.5 0.1 230), $method: oklch shorter hue)}\n"
        ),
        "a {\
         \n  b: oklch(50% 0.1 310deg);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn weighted() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(red, green, 20%, lch longer hue)}\n"),
        "a {\
         \n  b: rgb(201.9125152451, 62.5456438786, 25.0531427989);\
         \n}\n"
    );
}
