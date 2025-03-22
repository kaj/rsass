//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/error.hrx"

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
             \na {b: color.channel(red, \"foo\")}\n"
            ),
            "Error: $channel: Color red has no channel named foo.\
         \n  ,\
         \n2 | a {b: color.channel(red, \"foo\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.channel(hsl(50deg 50% 50%), hue)}\n"
            ),
            "Error: $channel: Expected hue to be a quoted string.\
         \n  ,\
         \n2 | a {b: color.channel(hsl(50deg 50% 50%), hue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.channel(red, \"RED\")}\n"
            ),
            "Error: $channel: Color red has no channel named RED.\
         \n  ,\
         \n2 | a {b: color.channel(red, \"RED\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.channel(red, \"hue\")}\n"
            ),
            "Error: $channel: Color red has no channel named hue.\
         \n  ,\
         \n2 | a {b: color.channel(red, \"hue\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.channel(red, \"hue\", $space: \"hsl\")}\n"
            ),
            "Error: $space: Expected \"hsl\" to be an unquoted string.\
         \n  ,\
         \n2 | a {b: color.channel(red, \"hue\", $space: \"hsl\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.channel(red, \"hue\", $space: \"foo\")}\n"
            ),
            "Error: $space: Expected \"foo\" to be an unquoted string.\
         \n  ,\
         \n2 | a {b: color.channel(red, \"hue\", $space: \"foo\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.channel(pink)}\n"
        ),
        "Error: Missing argument $channel.\
         \n  ,--> input.scss\
         \n2 | a {b: color.channel(pink)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function channel($color, $channel, $space: null) {\
         \n  |           ======================================= declaration\
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
             \na {b: color.channel(pink, \"red\", rgb, \"red\")}\n"
        ),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.channel(pink, \"red\", rgb, \"red\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function channel($color, $channel, $space: null) {\
         \n  |           ======================================= declaration\
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
             \na {b: color.channel(pink, red)}\n"
            ),
            "Error: $channel: red is not a string.\
         \n  ,\
         \n2 | a {b: color.channel(pink, red)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: color.channel(\"pink\", \"red\")}\n"
            ),
            "Error: $color: \"pink\" is not a color.\
         \n  ,\
         \n2 | a {b: color.channel(\"pink\", \"red\")}\
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
             \na {b: color.channel(pink, \"red\", red)}\n"
            ),
            "Error: $space: red is not a string.\
         \n  ,\
         \n2 | a {b: color.channel(pink, \"red\", red)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
