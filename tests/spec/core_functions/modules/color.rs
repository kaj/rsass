//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn adjust() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#abcdef, $red: 10)}\n"),
        "a {\
         \n  b: #b5cdef;\
         \n}\n"
    );
}
#[test]
fn alpha() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.alpha(#abcdef)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn blue() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.blue(#abcdef)}\n"),
        "a {\
         \n  b: 239;\
         \n}\n"
    );
}
#[test]
fn change() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.change(#abcdef, $red: 10)}\n"),
        "a {\
         \n  b: #0acdef;\
         \n}\n"
    );
}
#[test]
fn complement() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.complement(#abcdef)}\n"),
        "a {\
         \n  b: #efcdab;\
         \n}\n"
    );
}
mod css_overloads {
    #[allow(unused)]
    use super::runner;
    mod alpha {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn multi_arg() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.alpha(c=d, e=f, g=h)}\n"),
                "a {\
         \n  b: alpha(c=d, e=f, g=h);\
         \n}\n"
            );
        }
        #[test]
        fn one_arg() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.alpha(c=d)}\n"),
                "a {\
         \n  b: alpha(c=d);\
         \n}\n"
            );
        }
    }
    #[test]
    fn grayscale() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(1)}\n"),
            "a {\
         \n  b: grayscale(1);\
         \n}\n"
        );
    }
    #[test]
    fn invert() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.invert(1)}\n"),
            "a {\
         \n  b: invert(1);\
         \n}\n"
        );
    }
    #[test]
    fn opacity() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.opacity(1)}\n"),
            "a {\
         \n  b: opacity(1);\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn adjust_color() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.adjust-color(#abcdef, $red: 10)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: color.adjust-color(#abcdef, $red: 10)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn adjust_hue() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust-hue(#abcdef, 10)}\n"
        ),
        "Error: The function adjust-hue() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust(#abcdef, $hue: 10)\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#adjust-hue\
         \n  ,\
         \n2 | a {b: color.adjust-hue(#abcdef, 10)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn change_color() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.change-color(#abcdef, $red: 10)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: color.change-color(#abcdef, $red: 10)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn darken() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.darken(#abcdef, 10%)}\n"
        ),
        "Error: The function darken() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust(#abcdef, $lightness: -10%)\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#darken\
         \n  ,\
         \n2 | a {b: color.darken(#abcdef, 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn desaturate() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.desaturate(#abcdef, 10%)}\n"
        ),
        "Error: The function desaturate() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust(#abcdef, $saturation: -10%)\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#desaturate\
         \n  ,\
         \n2 | a {b: color.desaturate(#abcdef, 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn fade_in() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.fade-in(#abcdef, 0.5)}\n"
        ),
        "Error: The function fade-in() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust(#abcdef, $alpha: 0.5)\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#fade-in\
         \n  ,\
         \n2 | a {b: color.fade-in(#abcdef, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn fade_out() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.fade-out(#abcdef, 0.5)}\n"
        ),
        "Error: The function fade-out() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust(#abcdef, $alpha: -0.5)\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#fade-out\
         \n  ,\
         \n2 | a {b: color.fade-out(#abcdef, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn lighten() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.lighten(#abcdef, 10%)}\n"
        ),
        "Error: The function lighten() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust(#abcdef, $lightness: 10%)\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#lighten\
         \n  ,\
         \n2 | a {b: color.lighten(#abcdef, 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn opacify() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.opacify(#abcdef, 0.5)}\n"
        ),
        "Error: The function opacify() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust(#abcdef, $alpha: 0.5)\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#opacify\
         \n  ,\
         \n2 | a {b: color.opacify(#abcdef, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn saturate() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.saturate(#abcdef, 10%)}\n"
        ),
        "Error: The function saturate() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust(#abcdef, $saturation: 10%)\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#saturate\
         \n  ,\
         \n2 | a {b: color.saturate(#abcdef, 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn scale_color() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale-color(#abcdef, $red: 10%)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: color.scale-color(#abcdef, $red: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn transparentize() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.transparentize(#abcdef, 0.5)}\n"
        ),
        "Error: The function transparentize() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust(#abcdef, $alpha: -0.5)\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#transparentize\
         \n  ,\
         \n2 | a {b: color.transparentize(#abcdef, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
#[test]
fn green() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.green(#abcdef)}\n"),
        "a {\
         \n  b: 205;\
         \n}\n"
    );
}
#[test]
fn hue() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hue(#abcdef)}\n"),
        "a {\
         \n  b: 210deg;\
         \n}\n"
    );
}
#[test]
fn ie_hex_str() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.ie-hex-str(#abcdef)}\n"),
        "a {\
         \n  b: #FFABCDEF;\
         \n}\n"
    );
}
#[test]
fn invert() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.invert(#abcdef)}\n"),
        "a {\
         \n  b: #543210;\
         \n}\n"
    );
}
#[test]
fn lightness() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.lightness(#abcdef)}\n"),
        "a {\
         \n  b: 80.3921568627%;\
         \n}\n"
    );
}
#[test]
fn mix() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#abcdef, #daddee)}\n"),
        "a {\
         \n  b: #c3d5ef;\
         \n}\n"
    );
}
#[test]
fn red() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.red(#abcdef)}\n"),
        "a {\
         \n  b: 171;\
         \n}\n"
    );
}
#[test]
fn saturation() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.saturation(#abcdef)}\n"),
        "a {\
         \n  b: 68%;\
         \n}\n"
    );
}
#[test]
fn scale() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.scale(#abcdef, $red: 10%)}\n"),
        "a {\
         \n  b: #b3cdef;\
         \n}\n"
    );
}
