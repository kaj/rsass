//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color.hrx"

#[test]
fn adjust() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.adjust(#abcdef, $red: 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #b5cdef;\
        \n}\
        \n"
    );
}
#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.alpha(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn blue() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.blue(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 239;\
        \n}\
        \n"
    );
}
#[test]
fn change() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.change(#abcdef, $red: 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #0acdef;\
        \n}\
        \n"
    );
}
#[test]
fn complement() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.complement(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #efcdab;\
        \n}\
        \n"
    );
}
mod css_overloads {
    mod alpha {
        #[test]
        fn multi_arg() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:color\";\
            \na {b: color.alpha(c=d, e=f, g=h)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: alpha(c=d, e=f, g=h);\
        \n}\
        \n"
            );
        }
        #[test]
        fn one_arg() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:color\";\
            \na {b: color.alpha(c=d)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: alpha(c=d);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn grayscale() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
            \na {b: color.grayscale(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: grayscale(1);\
        \n}\
        \n"
        );
    }
    #[test]
    fn invert() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
            \na {b: color.invert(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: invert(1);\
        \n}\
        \n"
        );
    }
    #[test]
    fn opacity() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
            \na {b: color.opacity(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: opacity(1);\
        \n}\
        \n"
        );
    }
}
mod error {
    #[test]
    fn adjust_color() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
             \na {b: color.adjust-color(#abcdef, $red: 10)}\
             \n"
            )
            .unwrap_err(),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: color.adjust-color(#abcdef, $red: 10)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn adjust_hue() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
             \na {b: color.adjust-hue(#abcdef, 10)}\
             \n"
        ).unwrap_err(),
        "Error: The function adjust-hue() isn\'t in the sass:color module.\
         \n\
         \nRecommendation: color.adjust(#abcdef, $hue: 10)\
         \n\
         \nMore info: https://sass-lang.com/documentation/functions/color#adjust-hue\
         \n  ,\
         \n2 | a {b: color.adjust-hue(#abcdef, 10)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
    }
    #[test]
    fn change_color() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
             \na {b: color.change-color(#abcdef, $red: 10)}\
             \n"
            )
            .unwrap_err(),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: color.change-color(#abcdef, $red: 10)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn darken() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
             \na {b: color.darken(#abcdef, 10%)}\
             \n"
        ).unwrap_err(),
        "Error: The function darken() isn\'t in the sass:color module.\
         \n\
         \nRecommendation: color.adjust(#abcdef, $lightness: -10%)\
         \n\
         \nMore info: https://sass-lang.com/documentation/functions/color#darken\
         \n  ,\
         \n2 | a {b: color.darken(#abcdef, 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
    }
    #[test]
    #[ignore] // missing error
    fn desaturate() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
             \na {b: color.desaturate(#abcdef, 10%)}\
             \n"
        ).unwrap_err(),
        "Error: The function desaturate() isn\'t in the sass:color module.\
         \n\
         \nRecommendation: color.adjust(#abcdef, $saturation: -10%)\
         \n\
         \nMore info: https://sass-lang.com/documentation/functions/color#desaturate\
         \n  ,\
         \n2 | a {b: color.desaturate(#abcdef, 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
    }
    #[test]
    #[ignore] // missing error
    fn fade_in() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
             \na {b: color.fade-in(#abcdef, 0.5)}\
             \n"
        ).unwrap_err(),
        "Error: The function fade-in() isn\'t in the sass:color module.\
         \n\
         \nRecommendation: color.adjust(#abcdef, $alpha: 0.5)\
         \n\
         \nMore info: https://sass-lang.com/documentation/functions/color#fade-in\
         \n  ,\
         \n2 | a {b: color.fade-in(#abcdef, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
    }
    #[test]
    #[ignore] // missing error
    fn fade_out() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
             \na {b: color.fade-out(#abcdef, 0.5)}\
             \n"
        ).unwrap_err(),
        "Error: The function fade-out() isn\'t in the sass:color module.\
         \n\
         \nRecommendation: color.adjust(#abcdef, $alpha: -0.5)\
         \n\
         \nMore info: https://sass-lang.com/documentation/functions/color#fade-out\
         \n  ,\
         \n2 | a {b: color.fade-out(#abcdef, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
    }
    #[test]
    #[ignore] // missing error
    fn lighten() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
             \na {b: color.lighten(#abcdef, 10%)}\
             \n"
        ).unwrap_err(),
        "Error: The function lighten() isn\'t in the sass:color module.\
         \n\
         \nRecommendation: color.adjust(#abcdef, $lightness: 10%)\
         \n\
         \nMore info: https://sass-lang.com/documentation/functions/color#lighten\
         \n  ,\
         \n2 | a {b: color.lighten(#abcdef, 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn opacify() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
             \na {b: color.opacify(#abcdef, 0.5)}\
             \n"
        ).unwrap_err(),
        "Error: The function opacify() isn\'t in the sass:color module.\
         \n\
         \nRecommendation: color.adjust(#abcdef, $alpha: 0.5)\
         \n\
         \nMore info: https://sass-lang.com/documentation/functions/color#opacify\
         \n  ,\
         \n2 | a {b: color.opacify(#abcdef, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
    }
    #[test]
    #[ignore] // missing error
    fn saturate() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
             \na {b: color.saturate(#abcdef, 10%)}\
             \n"
        ).unwrap_err(),
        "Error: The function saturate() isn\'t in the sass:color module.\
         \n\
         \nRecommendation: color.adjust(#abcdef, $saturation: 10%)\
         \n\
         \nMore info: https://sass-lang.com/documentation/functions/color#saturate\
         \n  ,\
         \n2 | a {b: color.saturate(#abcdef, 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
    }
    #[test]
    fn scale_color() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
             \na {b: color.scale-color(#abcdef, $red: 10%)}\
             \n"
            )
            .unwrap_err(),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: color.scale-color(#abcdef, $red: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn transparentize() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
             \na {b: color.transparentize(#abcdef, 0.5)}\
             \n"
        ).unwrap_err(),
        "Error: The function transparentize() isn\'t in the sass:color module.\
         \n\
         \nRecommendation: color.adjust(#abcdef, $alpha: -0.5)\
         \n\
         \nMore info: https://sass-lang.com/documentation/functions/color#transparentize\
         \n  ,\
         \n2 | a {b: color.transparentize(#abcdef, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
    }
}
#[test]
fn green() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.green(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 205;\
        \n}\
        \n"
    );
}
#[test]
fn hue() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.hue(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 210deg;\
        \n}\
        \n"
    );
}
#[test]
fn ie_hex_str() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.ie-hex-str(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #FFABCDEF;\
        \n}\
        \n"
    );
}
#[test]
fn invert() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.invert(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #543210;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn lightness() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.lightness(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 80.3921568627%;\
        \n}\
        \n"
    );
}
#[test]
fn mix() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.mix(#abcdef, #daddee)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #c3d5ef;\
        \n}\
        \n"
    );
}
#[test]
fn red() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.red(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 171;\
        \n}\
        \n"
    );
}
#[test]
fn saturation() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.saturation(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 68%;\
        \n}\
        \n"
    );
}
#[test]
fn scale() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.scale(#abcdef, $red: 10%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #b3cdef;\
        \n}\
        \n"
    );
}
