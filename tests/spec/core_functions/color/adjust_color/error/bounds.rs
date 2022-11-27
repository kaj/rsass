//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_color/error/bounds.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bounds")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
            runner().err("a {b: adjust-color(red, $alpha: 1.001)}\n"),
            "Error: $alpha: Expected 1.001 to be within -1 and 1.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $alpha: 1.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: adjust-color(red, $alpha: -1.001)}\n"),
            "Error: $alpha: Expected -1.001 to be within -1 and 1.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $alpha: -1.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn unit() {
        assert_eq!(
        runner().err(
            "// This test covers sass/dart-sass#1745, but should be removed once units are\
             \n// fully forbidden (sass/sass#3374).\
             \na {b: adjust-color(red, $alpha: 50%)}\n"
        ),
        "DEPRECATION WARNING: $alpha: Passing a number with unit % is deprecated.\n\
         \nTo preserve current behavior: calc($alpha / 1%)\n\
         \nMore info: https://sass-lang.com/d/function-units\n\
         \n  ,\
         \n3 | a {b: adjust-color(red, $alpha: 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 3:7  root stylesheet\n\
         \nError: $alpha: Expected 50% to be within -1 and 1.\
         \n  ,\
         \n3 | a {b: adjust-color(red, $alpha: 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
}
mod blackness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
        runner().err(
            "a {b: adjust-color(red, $blackness: 100.001%)}\n"
        ),
        "Error: $blackness: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $blackness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_low() {
        assert_eq!(
        runner().err(
            "a {b: adjust-color(red, $blackness: -100.001%)}\n"
        ),
        "Error: $blackness: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $blackness: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
mod blue {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
            runner().err("a {b: adjust-color(blue, $blue: 256)}\n"),
            "Error: $blue: Expected 256 to be within -255 and 255.\
         \n  ,\
         \n1 | a {b: adjust-color(blue, $blue: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: adjust-color(blue, $blue: -256)}\n"),
            "Error: $blue: Expected -256 to be within -255 and 255.\
         \n  ,\
         \n1 | a {b: adjust-color(blue, $blue: -256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
        runner().err(
            "// This test covers sass/dart-sass#1745, but should be removed once units are\
             \n// fully forbidden (sass/sass#3374).\
             \na {b: adjust-color(blue, $blue: 300px)}\n"
        ),
        "Error: $blue: Expected 300px to be within -255 and 255.\
         \n  ,\
         \n3 | a {b: adjust-color(blue, $blue: 300px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
}
mod green {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
            runner().err("a {b: adjust-color(green, $green: 256)}\n"),
            "Error: $green: Expected 256 to be within -255 and 255.\
         \n  ,\
         \n1 | a {b: adjust-color(green, $green: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: adjust-color(green, $green: -256)}\n"),
            "Error: $green: Expected -256 to be within -255 and 255.\
         \n  ,\
         \n1 | a {b: adjust-color(green, $green: -256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
        runner().err(
            "// This test covers sass/dart-sass#1745, but should be removed once units are\
             \n// fully forbidden (sass/sass#3374).\
             \na {b: adjust-color(green, $green: 300px)}\n"
        ),
        "Error: $green: Expected 300px to be within -255 and 255.\
         \n  ,\
         \n3 | a {b: adjust-color(green, $green: 300px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
        runner().err(
            "a {b: adjust-color(red, $lightness: 100.001%)}\n"
        ),
        "Error: $lightness: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $lightness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_low() {
        assert_eq!(
        runner().err(
            "a {b: adjust-color(red, $lightness: -100.001%)}\n"
        ),
        "Error: $lightness: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $lightness: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn unit() {
        assert_eq!(
        runner().err(
            "// This test covers sass/dart-sass#1745, but should be removed once units are\
             \n// fully forbidden (sass/sass#3374).\
             \na {b: adjust-color(red, $lightness: 200px)}\n"
        ),
        "DEPRECATION WARNING: $lightness: Passing a number without unit % (200px) is deprecated.\n\
         \nTo preserve current behavior: calc($lightness / 1px * 1%)\n\
         \nMore info: https://sass-lang.com/d/function-units\n\
         \n  ,\
         \n3 | a {b: adjust-color(red, $lightness: 200px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 3:7  root stylesheet\n\
         \nError: $lightness: Expected 200px to be within -100% and 100%.\
         \n  ,\
         \n3 | a {b: adjust-color(red, $lightness: 200px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
}
mod red {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
            runner().err("a {b: adjust-color(red, $red: 256)}\n"),
            "Error: $red: Expected 256 to be within -255 and 255.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $red: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: adjust-color(red, $red: -256)}\n"),
            "Error: $red: Expected -256 to be within -255 and 255.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $red: -256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
        runner().err(
            "// This test covers sass/dart-sass#1745, but should be removed once units are\
             \n// fully forbidden (sass/sass#3374).\
             \na {b: adjust-color(red, $red: 300px)}\n"
        ),
        "Error: $red: Expected 300px to be within -255 and 255.\
         \n  ,\
         \n3 | a {b: adjust-color(red, $red: 300px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
}
mod saturation {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
        runner().err(
            "a {b: adjust-color(red, $saturation: 100.001%)}\n"
        ),
        "Error: $saturation: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $saturation: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_low() {
        assert_eq!(
        runner().err(
            "a {b: adjust-color(red, $saturation: -100.001%)}\n"
        ),
        "Error: $saturation: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $saturation: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn unit() {
        assert_eq!(
        runner().err(
            "// This test covers sass/dart-sass#1745, but should be removed once units are\
             \n// fully forbidden (sass/sass#3374).\
             \na {b: adjust-color(red, $saturation: 200px)}\n"
        ),
        "DEPRECATION WARNING: $saturation: Passing a number without unit % (200px) is deprecated.\n\
         \nTo preserve current behavior: calc($saturation / 1px * 1%)\n\
         \nMore info: https://sass-lang.com/d/function-units\n\
         \n  ,\
         \n3 | a {b: adjust-color(red, $saturation: 200px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 3:7  root stylesheet\n\
         \nError: $saturation: Expected 200px to be within -100% and 100%.\
         \n  ,\
         \n3 | a {b: adjust-color(red, $saturation: 200px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
}
mod whiteness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_high() {
        assert_eq!(
        runner().err(
            "a {b: adjust-color(red, $whiteness: 100.001%)}\n"
        ),
        "Error: $whiteness: Expected 100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $whiteness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_low() {
        assert_eq!(
        runner().err(
            "a {b: adjust-color(red, $whiteness: -100.001%)}\n"
        ),
        "Error: $whiteness: Expected -100.001% to be within -100% and 100%.\
         \n  ,\
         \n1 | a {b: adjust-color(red, $whiteness: -100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
