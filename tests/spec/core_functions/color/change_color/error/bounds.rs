//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/error/bounds.hrx"

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
            runner().err("a {b: change-color(red, $alpha: 1.001)}\n"),
            "Error: $alpha: Expected 1.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: change-color(red, $alpha: 1.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $alpha: -0.001)}\n"),
            "Error: $alpha: Expected -0.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: change-color(red, $alpha: -0.001)}\
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
             \na {b: change-color(red, $alpha: 50%)}\n"
        ),
        "Error: $alpha: Expected 50% to be within 0 and 1.\
         \n  ,\
         \n3 | a {b: change-color(red, $alpha: 50%)}\
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
            runner().err("a {b: change-color(red, $blackness: 100.001%)}\n"),
            "Error: $blackness: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $blackness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $blackness: -0.001%)}\n"),
            "Error: $blackness: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $blackness: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
            runner().err("a {b: change-color(blue, $blue: 256)}\n"),
            "Error: $blue: Expected 256 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(blue, $blue: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(blue, $blue: -1)}\n"),
            "Error: $blue: Expected -1 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(blue, $blue: -1)}\
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
             \na {b: change-color(blue, $blue: 300px)}\n"
        ),
        "Error: $blue: Expected 300px to be within 0 and 255.\
         \n  ,\
         \n3 | a {b: change-color(blue, $blue: 300px)}\
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
            runner().err("a {b: change-color(green, $green: 256)}\n"),
            "Error: $green: Expected 256 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(green, $green: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(green, $green: -1)}\n"),
            "Error: $green: Expected -1 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(green, $green: -1)}\
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
             \na {b: change-color(green, $green: 300px)}\n"
        ),
        "Error: $green: Expected 300px to be within 0 and 255.\
         \n  ,\
         \n3 | a {b: change-color(green, $green: 300px)}\
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
            runner().err("a {b: change-color(red, $lightness: 100.001%)}\n"),
            "Error: $lightness: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $lightness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $lightness: -0.001%)}\n"),
            "Error: $lightness: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $lightness: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: change-color(red, $lightness: 200px)}\n"
        ),
        "DEPRECATION WARNING: $lightness: Passing a number without unit % (200px) is deprecated.\n\
         \nTo preserve current behavior: $lightness / 1px * 1%\n\
         \n  ,\
         \n3 | a {b: change-color(red, $lightness: 200px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 3:7  root stylesheet\n\
         \nError: $lightness: Expected 200px to be within 0% and 100%.\
         \n  ,\
         \n3 | a {b: change-color(red, $lightness: 200px)}\
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
            runner().err("a {b: change-color(red, $red: 256)}\n"),
            "Error: $red: Expected 256 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(red, $red: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $red: -1)}\n"),
            "Error: $red: Expected -1 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(red, $red: -1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: change-color(red, $red: 300px)}\n"
        ),
        "Error: $red: Expected 300px to be within 0 and 255.\
         \n  ,\
         \n3 | a {b: change-color(red, $red: 300px)}\
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
            runner().err("a {b: change-color(red, $saturation: 100.001%)}\n"),
            "Error: $saturation: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $saturation: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $saturation: -0.001%)}\n"),
            "Error: $saturation: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $saturation: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: change-color(red, $saturation: 200px)}\n"
        ),
        "DEPRECATION WARNING: $saturation: Passing a number without unit % (200px) is deprecated.\n\
         \nTo preserve current behavior: $saturation / 1px * 1%\n\
         \n  ,\
         \n3 | a {b: change-color(red, $saturation: 200px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 3:7  root stylesheet\n\
         \nError: $saturation: Expected 200px to be within 0% and 100%.\
         \n  ,\
         \n3 | a {b: change-color(red, $saturation: 200px)}\
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
            runner().err("a {b: change-color(red, $whiteness: 100.001%)}\n"),
            "Error: $whiteness: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $whiteness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_low() {
        assert_eq!(
            runner().err("a {b: change-color(red, $whiteness: -0.001%)}\n"),
            "Error: $whiteness: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $whiteness: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
