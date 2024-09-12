//! Tests auto-converted from "sass-spec/spec/core_functions/color/is_in_gamut.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("is_in_gamut")
        .mock_file("error/too_few_args/error ", "")
}

#[test]
#[ignore] // unexepected error
fn a98_rgb() {
    let runner = runner().with_cwd("a98-rgb");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(a98-rgb 0 1 -1))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn color_keyword() {
    let runner = runner().with_cwd("color_keyword");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(midnightblue)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn display_p3() {
    let runner = runner().with_cwd("display-p3");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(display-p3 0 2 0))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    #[ignore] // wrong error
    fn invalid_character_end() {
        let runner = runner().with_cwd("invalid_character_end");
        assert_eq!(
            runner.err(
                "@use \"sass:color\";\
             \na {b: color.is-in-gamut(rgb(0 255 0), hsl.)}\n"
            ),
            "Error: Expected identifier.\
         \n  ,\
         \n2 | a {b: color.is-in-gamut(rgb(0 255 0), hsl.)}\
         \n  |                                           ^\
         \n  \'\
         \n  input.scss 2:43  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn invalid_character_start() {
        let runner = runner().with_cwd("invalid_character_start");
        assert_eq!(
            runner.err(
                "@use \"sass:color\";\
             \na {b: color.is-in-gamut(rgb(0 255 0), .hsl)}\n"
            ),
            "Error: Expected digit.\
         \n  ,\
         \n2 | a {b: color.is-in-gamut(rgb(0 255 0), .hsl)}\
         \n  |                                        ^\
         \n  \'\
         \n  input.scss 2:40  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn quoted() {
        let runner = runner().with_cwd("quoted");
        assert_eq!(
            runner.err(
                "@use \"sass:color\";\
             \na {b: color.is-in-gamut(rgb(0 255 0), \"hsl\")}\n"
            ),
            "Error: $space: Expected \"hsl\" to be an unquoted string.\
         \n  ,\
         \n2 | a {b: color.is-in-gamut(rgb(0 255 0), \"hsl\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn space_unknown() {
        let runner = runner().with_cwd("space_unknown");
        assert_eq!(
            runner.err(
                "@use \"sass:color\";\
             \na {b: color.is-in-gamut(rgb(0 255 0), c)}\n"
            ),
            "Error: $space: Unknown color space \"c\".\
         \n  ,\
         \n2 | a {b: color.is-in-gamut(rgb(0 255 0), c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        let runner = runner().with_cwd("too_few_args");
        assert_eq!(
            runner.err(
                "@use \"sass:color\";\
             \na {b: color.is-in-gamut()}\n"
            ),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.is-in-gamut()}\
         \n  |       ^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function is-in-gamut($color, $space: null) {\
         \n  |           ================================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        let runner = runner().with_cwd("too_many_args");
        assert_eq!(
            runner.err(
                "@use \"sass:color\";\
             \na {b: color.is-in-gamut(rgb(0 255 0), hwb, c)}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.is-in-gamut(rgb(0 255 0), hwb, c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function is-in-gamut($color, $space: null) {\
         \n  |           ================================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        let runner = runner().with_cwd("type");
        assert_eq!(
            runner.err(
                "@use \"sass:color\";\
             \na {b: color.is-in-gamut(1)}\n"
            ),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.is-in-gamut(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
#[ignore] // unexepected error
fn hex() {
    let runner = runner().with_cwd("hex");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(#f2ece4)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn hsl() {
    let runner = runner().with_cwd("hsl");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(hsl(0 0% -1%))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn hwb() {
    let runner = runner().with_cwd("hwb");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(hwb(0 300% -1%))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn lab() {
    let runner = runner().with_cwd("lab");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(lab(5 110 115))}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn lch() {
    let runner = runner().with_cwd("lch");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(lch(90.6 52.8 197))}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok(
            "@use \"sass:color\";\
             \na {b: color.is-in-gamut($color: color(display-p3 1 1 0), $space: srgb)}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn narrow_wide() {
    let runner = runner().with_cwd("narrow_wide");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(hsl(50 40% 30%), oklab)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn oklab() {
    let runner = runner().with_cwd("oklab");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(oklab(0.44027 0.08818 -0.13386))}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn oklab_lch() {
    let runner = runner().with_cwd("oklab_lch");
    assert_eq!(
        runner.ok(
            "@use \"sass:color\";\
             \na {b: color.is-in-gamut(oklab(0.44027 0.08818 -0.13386), lch)}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn oklch() {
    let runner = runner().with_cwd("oklch");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(oklch(70% 0.1 200))}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn prophoto_rgb() {
    let runner = runner().with_cwd("prophoto-rgb");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(prophoto-rgb 2 0 0))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn rec2020() {
    let runner = runner().with_cwd("rec2020");
    assert_eq!(
        runner.ok(
            "@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(rec2020 0.979780 -1 0.00579))}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn rgb() {
    let runner = runner().with_cwd("rgb");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(color.change(black, $green: 300))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn rgba() {
    let runner = runner().with_cwd("rgba");
    assert_eq!(
        runner.ok(
            "@use \"sass:color\";\
             \na {b: color.is-in-gamut(color.change(rgba(0 122 180 / 0.4), $red: -1))}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn space() {
    let runner = runner().with_cwd("space");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(display-p3 1 1 0), oklch)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn srgb_linear() {
    let runner = runner().with_cwd("srgb-linear");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(srgb-linear -1 0 0))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn srgb() {
    let runner = runner().with_cwd("srgb");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(srgb 0 0 1.5))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn wide_narrow() {
    let runner = runner().with_cwd("wide_narrow");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(display-p3 1 1 0), hwb)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn xyz_d50() {
    let runner = runner().with_cwd("xyz-d50");
    assert_eq!(
        runner.ok(
            "@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(xyz-d50 0.2005 0.14089 0.4472))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn xyz_d65() {
    let runner = runner().with_cwd("xyz-d65");
    assert_eq!(
        runner.ok(
            "@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(xyz-d65 0.21661 0.14602 0.59452))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn xyz() {
    let runner = runner().with_cwd("xyz");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-in-gamut(color(xyz 0.0426 0.0442 0.0364))}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
