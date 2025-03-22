//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/known_incompatible/frequency.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("frequency")
}

mod hz {
    use super::runner;

    #[test]
    fn dpcm() {
        assert_eq!(
            runner().err("a {b: calc(1Hz + 1dpcm)}\n"),
            "Error: 1Hz and 1dpcm are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1Hz + 1dpcm)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dpi() {
        assert_eq!(
            runner().err("a {b: calc(1Hz + 1dpi)}\n"),
            "Error: 1Hz and 1dpi are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1Hz + 1dpi)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dppx() {
        assert_eq!(
            runner().err("a {b: calc(1Hz + 1dppx)}\n"),
            "Error: 1Hz and 1dppx are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1Hz + 1dppx)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
mod khz {
    use super::runner;

    #[test]
    fn dpcm() {
        assert_eq!(
            runner().err("a {b: calc(1kHz + 1dpcm)}\n"),
            "Error: 1kHz and 1dpcm are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1kHz + 1dpcm)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dpi() {
        assert_eq!(
            runner().err("a {b: calc(1kHz + 1dpi)}\n"),
            "Error: 1kHz and 1dpi are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1kHz + 1dpi)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    fn dppx() {
        assert_eq!(
            runner().err("a {b: calc(1kHz + 1dppx)}\n"),
            "Error: 1kHz and 1dppx are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1kHz + 1dppx)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
