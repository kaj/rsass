//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/known_incompatible/time.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("time")
}

mod ms {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn dpcm() {
        assert_eq!(
            runner().err("a {b: calc(1ms + 1dpcm)}\n"),
            "Error: 1ms and 1dpcm are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ms + 1dpcm)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dpi() {
        assert_eq!(
            runner().err("a {b: calc(1ms + 1dpi)}\n"),
            "Error: 1ms and 1dpi are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ms + 1dpi)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dppx() {
        assert_eq!(
            runner().err("a {b: calc(1ms + 1dppx)}\n"),
            "Error: 1ms and 1dppx are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ms + 1dppx)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn hz() {
        assert_eq!(
            runner().err("a {b: calc(1ms + 1Hz)}\n"),
            "Error: 1ms and 1Hz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ms + 1Hz)}\
         \n  |            ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn khz() {
        assert_eq!(
            runner().err("a {b: calc(1ms + 1kHz)}\n"),
            "Error: 1ms and 1kHz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ms + 1kHz)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
mod s {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn dpcm() {
        assert_eq!(
            runner().err("a {b: calc(1s + 1dpcm)}\n"),
            "Error: 1s and 1dpcm are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1s + 1dpcm)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dpi() {
        assert_eq!(
            runner().err("a {b: calc(1s + 1dpi)}\n"),
            "Error: 1s and 1dpi are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1s + 1dpi)}\
         \n  |            ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dppx() {
        assert_eq!(
            runner().err("a {b: calc(1s + 1dppx)}\n"),
            "Error: 1s and 1dppx are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1s + 1dppx)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn hz() {
        assert_eq!(
            runner().err("a {b: calc(1s + 1Hz)}\n"),
            "Error: 1s and 1Hz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1s + 1Hz)}\
         \n  |            ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn khz() {
        assert_eq!(
            runner().err("a {b: calc(1s + 1kHz)}\n"),
            "Error: 1s and 1kHz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1s + 1kHz)}\
         \n  |            ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
