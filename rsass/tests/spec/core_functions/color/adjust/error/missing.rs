//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/error/missing.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("missing")
}

#[test]
#[ignore] // wrong error
fn alpha() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(rgb(0 0 0 / none), $alpha: 0.1)}\n"
        ),
        "Error: $alpha: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: rgb(0 0 0 / none)).\
         \n  ,\
         \n2 | a {b: color.adjust(rgb(0 0 0 / none), $alpha: 0.1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn legacy() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(rgb(none 0 0), $red: 10)}\n"
        ),
        "Error: $red: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: rgb(none 0 0)).\
         \n  ,\
         \n2 | a {b: color.adjust(rgb(none 0 0), $red: 10)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn modern() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(lab(none 0 0), $lightness: 10%)}\n"
        ),
        "Error: $lightness: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: lab(none 0 0)).\
         \n  ,\
         \n2 | a {b: color.adjust(lab(none 0 0), $lightness: 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod powerless {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn legacy() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(grey, $hue: 10deg, $space: hsl)}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: hsl(none 0% 50.1960784314%)).\
         \n  ,\
         \n2 | a {b: color.adjust(grey, $hue: 10deg, $space: hsl)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn modern() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.adjust(grey, $hue: 10deg, $space: lch)}\n"
        ),
        "Error: $hue: Because the CSS working group is still deciding on the best behavior, Sass doesn\'t currently support modifying missing channels (color: lch(53.5850134522% 0 none)).\
         \n  ,\
         \n2 | a {b: color.adjust(grey, $hue: 10deg, $space: lch)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
