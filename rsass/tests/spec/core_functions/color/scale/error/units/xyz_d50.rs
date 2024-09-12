//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/units/xyz_d50.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz_d50")
}

mod none {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn x() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $x: 1)}\n"
            ),
            "Error: $x: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $x: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn y() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $y: 1)}\n"
            ),
            "Error: $y: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $y: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn z() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $z: 1)}\n"
            ),
            "Error: $z: Expected 1 to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $z: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod wrong {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn x() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $x: 1px)}\n"
            ),
            "Error: $x: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $x: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn y() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $y: 1px)}\n"
            ),
            "Error: $y: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $y: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn z() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $z: 1px)}\n"
            ),
            "Error: $z: Expected 1px to have unit \"%\".\
         \n  ,\
         \n2 | a {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $z: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
