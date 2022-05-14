//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/known_incompatible/length/em.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("em")
}

#[test]
fn deg() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1deg)}\n"),
        "Error: 1em and 1deg are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1deg)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn dpcm() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1dpcm)}\n"),
        "Error: 1em and 1dpcm are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1dpcm)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn dpi() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1dpi)}\n"),
        "Error: 1em and 1dpi are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1dpi)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn dppx() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1dppx)}\n"),
        "Error: 1em and 1dppx are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1dppx)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn grad() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1grad)}\n"),
        "Error: 1em and 1grad are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1grad)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn hz() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1Hz)}\n"),
        "Error: 1em and 1Hz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1Hz)}\
         \n  |            ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn khz() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1kHz)}\n"),
        "Error: 1em and 1kHz are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1kHz)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn ms() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1ms)}\n"),
        "Error: 1em and 1ms are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1ms)}\
         \n  |            ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn rad() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1rad)}\n"),
        "Error: 1em and 1rad are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1rad)}\
         \n  |            ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn s() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1s)}\n"),
        "Error: 1em and 1s are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1s)}\
         \n  |            ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn turn() {
    assert_eq!(
        runner().err("a {b: calc(1em + 1turn)}\n"),
        "Error: 1em and 1turn are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1em + 1turn)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
