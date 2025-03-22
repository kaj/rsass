//! Tests auto-converted from "sass-spec/spec/core_functions/color/is_powerless/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod channel {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn unknown() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.is-powerless(black, \"c\")}\n"
        ),
        "Error: $channel: Color black doesn\'t have a channel named \"c\".\
         \n  ,\
         \n2 | a {b: color.is-powerless(black, \"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn unquoted() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.is-powerless(hsl(0deg 0% 0%), hue)}\n"
            ),
            "Error: $channel: Expected hue to be a quoted string.\
         \n  ,\
         \n2 | a {b: color.is-powerless(hsl(0deg 0% 0%), hue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn wrong_case() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.is-powerless(black, \"RED\")}\n"
        ),
        "Error: $channel: Color black doesn\'t have a channel named \"RED\".\
         \n  ,\
         \n2 | a {b: color.is-powerless(black, \"RED\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn wrong_space() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.is-powerless(black, \"hue\")}\n"
        ),
        "Error: $channel: Color black doesn\'t have a channel named \"hue\".\
         \n  ,\
         \n2 | a {b: color.is-powerless(black, \"hue\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
mod space {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn quoted() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.is-powerless(black, \"red\", $space: \"rgb\")}\n"
            ),
            "Error: $space: Expected \"rgb\" to be an unquoted string.\
         \n  ,\
         \n2 | a {b: color.is-powerless(black, \"red\", $space: \"rgb\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn unknown() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.is-powerless(black, \"red\", $space: c)}\n"
            ),
            "Error: $space: Unknown color space \"c\".\
         \n  ,\
         \n2 | a {b: color.is-powerless(black, \"red\", $space: c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn too_few_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.is-powerless(black)}\n"
        ),
        "Error: Missing argument $channel.\
         \n  ,--> input.scss\
         \n2 | a {b: color.is-powerless(black)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function is-powerless($color, $channel, $space: null) {\
         \n  |           ============================================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn too_many_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.is-powerless(black, \"red\", rgb, c)}\n"
        ),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.is-powerless(black, \"red\", rgb, c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function is-powerless($color, $channel, $space: null) {\
         \n  |           ============================================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod test_type {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn channel() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.is-powerless(black, 1)}\n"
            ),
            "Error: $channel: 1 is not a string.\
         \n  ,\
         \n2 | a {b: color.is-powerless(black, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn color() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.is-powerless(c, \"red\")}\n"
            ),
            "Error: $color: c is not a color.\
         \n  ,\
         \n2 | a {b: color.is-powerless(c, \"red\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn space() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.is-powerless(black, \"red\", $space: 1)}\n"
            ),
            "Error: $space: 1 is not a string.\
         \n  ,\
         \n2 | a {b: color.is-powerless(black, \"red\", $space: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
