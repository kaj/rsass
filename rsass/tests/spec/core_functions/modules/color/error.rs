//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

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
