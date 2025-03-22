//! Tests auto-converted from "sass-spec/spec/core_functions/color/is_legacy.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("is_legacy")
        .mock_file("error/too_few_args/error ", "")
}

#[test]
#[ignore] // unexepected error
fn a98_rgb() {
    let runner = runner().with_cwd("a98-rgb");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-legacy(color(a98-rgb 0 1 0))}\n"),
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
             \na {b: color.is-legacy(midnightblue)}\n"),
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
        runner.ok(
            "@use \"sass:color\";\
             \na {b: color.is-legacy(color(display-p3 0.515 0.35 0.3 / 1))}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod error {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        let runner = runner().with_cwd("too_few_args");
        assert_eq!(
            runner.err(
                "@use \"sass:color\";\
             \na {b: color.is-legacy()}\n"
            ),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.is-legacy()}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function is-legacy($color) {\
         \n  |           ================= declaration\
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
             \na {b: color.is-legacy(rgb(0 255 0), a)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.is-legacy(rgb(0 255 0), a)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function is-legacy($color) {\
         \n  |           ================= declaration\
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
             \na {b: color.is-legacy(1)}\n"
            ),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.is-legacy(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.is-legacy(#f2ece4)}\n"),
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
             \na {b: color.is-legacy(hsl(110 31% 32%))}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn hwb() {
    let runner = runner().with_cwd("hwb");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-legacy(hwb(0 50% 0%))}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn lab() {
    let runner = runner().with_cwd("lab");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-legacy(lab(5 110 115))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn lch() {
    let runner = runner().with_cwd("lch");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-legacy(lch(90.6 52.8 197))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-legacy($color: rgb(255 0 0))}\n"),
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
             \na {b: color.is-legacy(oklab(0.44027 0.08818 -0.13386))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn oklch() {
    let runner = runner().with_cwd("oklch");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-legacy(oklch(70% 0.1 200))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn prophoto_rgb() {
    let runner = runner().with_cwd("prophoto-rgb");
    assert_eq!(
        runner.ok(
            "@use \"sass:color\";\
             \na {b: color.is-legacy(color(prophoto-rgb 0.42444 0.934918 0.190922))}\n"
        ),
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
             \na {b: color.is-legacy(color(rec2020 0.42053 0.979780 0.00579))}\n"
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
             \na {b: color.is-legacy(rgb(0 255 0))}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn rgba() {
    let runner = runner().with_cwd("rgba");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-legacy(rgba(72 122 180 / .2))}\n"),
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
        runner.ok(
            "@use \"sass:color\";\
             \na {b: color.is-legacy(color(srgb-linear 0.45098 0.07843 0.823530))}\n"
        ),
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
        runner.ok(
            "@use \"sass:color\";\
             \na {b: color.is-legacy(color(srgb 0.45098 0.07843 0.823530))}\n"
        ),
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
             \na {b: color.is-legacy(color(xyz-d50 0.2005 0.14089 0.4472))}\n"
        ),
        "a {\
         \n  b: false;\
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
             \na {b: color.is-legacy(color(xyz-d65 0.21661 0.14602 0.59452))}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn xyz() {
    let runner = runner().with_cwd("xyz");
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {b: color.is-legacy(color(xyz 0.0426 0.0442 0.0364))}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
