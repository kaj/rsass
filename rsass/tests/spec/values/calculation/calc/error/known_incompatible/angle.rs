//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/known_incompatible/angle.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("angle")
}

mod deg {
    use super::runner;

    #[test]
    fn dpcm() {
        assert_eq!(
            runner().err("a {b: calc(1deg + 1dpcm)}\n"),
            "Error: 1deg and 1dpcm are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1deg + 1dpcm)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dpi() {
        assert_eq!(
            runner().err("a {b: calc(1deg + 1dpi)}\n"),
            "Error: 1deg and 1dpi are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1deg + 1dpi)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dppx() {
        assert_eq!(
            runner().err("a {b: calc(1deg + 1dppx)}\n"),
            "Error: 1deg and 1dppx are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1deg + 1dppx)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn hz() {
        assert_eq!(
            runner().err("a {b: calc(1deg + 1Hz)}\n"),
            "Error: 1deg and 1Hz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1deg + 1Hz)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn khz() {
        assert_eq!(
            runner().err("a {b: calc(1deg + 1kHz)}\n"),
            "Error: 1deg and 1kHz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1deg + 1kHz)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn ms() {
        assert_eq!(
            runner().err("a {b: calc(1deg + 1ms)}\n"),
            "Error: 1deg and 1ms are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1deg + 1ms)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn s() {
        assert_eq!(
            runner().err("a {b: calc(1deg + 1s)}\n"),
            "Error: 1deg and 1s are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1deg + 1s)}\
         \n  |            ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
mod grad {
    use super::runner;

    #[test]
    fn dpcm() {
        assert_eq!(
            runner().err("a {b: calc(1grad + 1dpcm)}\n"),
            "Error: 1grad and 1dpcm are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1grad + 1dpcm)}\
         \n  |            ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dpi() {
        assert_eq!(
            runner().err("a {b: calc(1grad + 1dpi)}\n"),
            "Error: 1grad and 1dpi are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1grad + 1dpi)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dppx() {
        assert_eq!(
            runner().err("a {b: calc(1grad + 1dppx)}\n"),
            "Error: 1grad and 1dppx are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1grad + 1dppx)}\
         \n  |            ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn hz() {
        assert_eq!(
            runner().err("a {b: calc(1grad + 1Hz)}\n"),
            "Error: 1grad and 1Hz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1grad + 1Hz)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn khz() {
        assert_eq!(
            runner().err("a {b: calc(1grad + 1kHz)}\n"),
            "Error: 1grad and 1kHz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1grad + 1kHz)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn ms() {
        assert_eq!(
            runner().err("a {b: calc(1grad + 1ms)}\n"),
            "Error: 1grad and 1ms are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1grad + 1ms)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn s() {
        assert_eq!(
            runner().err("a {b: calc(1grad + 1s)}\n"),
            "Error: 1grad and 1s are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1grad + 1s)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
mod rad {
    use super::runner;

    #[test]
    fn dpcm() {
        assert_eq!(
            runner().err("a {b: calc(1rad + 1dpcm)}\n"),
            "Error: 1rad and 1dpcm are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1rad + 1dpcm)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dpi() {
        assert_eq!(
            runner().err("a {b: calc(1rad + 1dpi)}\n"),
            "Error: 1rad and 1dpi are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1rad + 1dpi)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dppx() {
        assert_eq!(
            runner().err("a {b: calc(1rad + 1dppx)}\n"),
            "Error: 1rad and 1dppx are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1rad + 1dppx)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn hz() {
        assert_eq!(
            runner().err("a {b: calc(1rad + 1Hz)}\n"),
            "Error: 1rad and 1Hz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1rad + 1Hz)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn khz() {
        assert_eq!(
            runner().err("a {b: calc(1rad + 1kHz)}\n"),
            "Error: 1rad and 1kHz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1rad + 1kHz)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn ms() {
        assert_eq!(
            runner().err("a {b: calc(1rad + 1ms)}\n"),
            "Error: 1rad and 1ms are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1rad + 1ms)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn s() {
        assert_eq!(
            runner().err("a {b: calc(1rad + 1s)}\n"),
            "Error: 1rad and 1s are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1rad + 1s)}\
         \n  |            ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
mod turn {
    use super::runner;

    #[test]
    fn dpcm() {
        assert_eq!(
            runner().err("a {b: calc(1turn + 1dpcm)}\n"),
            "Error: 1turn and 1dpcm are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1turn + 1dpcm)}\
         \n  |            ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dpi() {
        assert_eq!(
            runner().err("a {b: calc(1turn + 1dpi)}\n"),
            "Error: 1turn and 1dpi are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1turn + 1dpi)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dppx() {
        assert_eq!(
            runner().err("a {b: calc(1turn + 1dppx)}\n"),
            "Error: 1turn and 1dppx are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1turn + 1dppx)}\
         \n  |            ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn hz() {
        assert_eq!(
            runner().err("a {b: calc(1turn + 1Hz)}\n"),
            "Error: 1turn and 1Hz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1turn + 1Hz)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn khz() {
        assert_eq!(
            runner().err("a {b: calc(1turn + 1kHz)}\n"),
            "Error: 1turn and 1kHz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1turn + 1kHz)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn ms() {
        assert_eq!(
            runner().err("a {b: calc(1turn + 1ms)}\n"),
            "Error: 1turn and 1ms are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1turn + 1ms)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn s() {
        assert_eq!(
            runner().err("a {b: calc(1turn + 1s)}\n"),
            "Error: 1turn and 1s are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1turn + 1s)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
