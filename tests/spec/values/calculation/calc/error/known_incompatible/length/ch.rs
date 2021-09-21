//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/known_incompatible/length/ch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn deg() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1deg)}\n"),
        "Error: 1ch and 1deg are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1deg)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn dpcm() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1dpcm)}\n"),
        "Error: 1ch and 1dpcm are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1dpcm)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn dpi() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1dpi)}\n"),
        "Error: 1ch and 1dpi are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1dpi)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn dppx() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1dppx)}\n"),
        "Error: 1ch and 1dppx are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1dppx)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn grad() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1grad)}\n"),
        "Error: 1ch and 1grad are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1grad)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn hz() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1Hz)}\n"),
        "Error: 1ch and 1Hz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1Hz)}\
         \n  |            ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn khz() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1kHz)}\n"),
        "Error: 1ch and 1kHz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1kHz)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn ms() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1ms)}\n"),
        "Error: 1ch and 1ms are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1ms)}\
         \n  |            ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn rad() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1rad)}\n"),
        "Error: 1ch and 1rad are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1rad)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn s() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1s)}\n"),
        "Error: 1ch and 1s are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1s)}\
         \n  |            ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn turn() {
    assert_eq!(
        runner().err("a {b: calc(1ch + 1turn)}\n"),
        "Error: 1ch and 1turn are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1ch + 1turn)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
