//! Tests auto-converted from "sass-spec/spec/core_functions/math/random.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("random")
        .mock_file("_util.scss", "@use \"sass:math\";\n@use \"sass:meta\";\n\n// Calls `random()` one thousand times, and throws an error if `$check` returns\n// `false` for any of the values.\n@mixin check-values($arg, $check) {\n  @for $i from 1 through 1000 {\n    $value: math.random($arg);\n    @if not meta.call($check, $value) {\n      @error \"#{$value} did not match expectation\";\n    }\n  }\n}\n")
}

mod error {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    fn decimal() {
        let runner = runner().with_cwd("decimal");
        assert_eq!(
            runner.err(
                "@use \"sass:math\";\
             \na {b: math.random(1.5)}\n"
            ),
            "Error: $limit: 1.5 is not an int.\
         \n  ,\
         \n2 | a {b: math.random(1.5)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn negative() {
        let runner = runner().with_cwd("negative");
        assert_eq!(
            runner.err(
                "@use \"sass:math\";\
             \na {b: math.random(-1)}\n"
            ),
            "Error: $limit: Must be greater than 0, was -1.\
         \n  ,\
         \n2 | a {b: math.random(-1)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        let runner = runner().with_cwd("type");
        assert_eq!(
            runner.err(
                "@use \"sass:math\";\
             \na {b: math.random(c)}\n"
            ),
            "Error: $limit: c is not a number.\
         \n  ,\
         \n2 | a {b: math.random(c)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn zero() {
        let runner = runner().with_cwd("zero");
        assert_eq!(
            runner.err(
                "@use \"sass:math\";\
             \na {b: math.random(0)}\n"
            ),
            "Error: $limit: Must be greater than 0, was 0.\
         \n  ,\
         \n2 | a {b: math.random(0)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn ignores_units() {
    let runner = runner().with_cwd("ignores_units");
    assert_eq!(
        runner.ok("@use \"sass:math\";\
             \na {b: math.random(1px)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok("@use \"sass:math\";\
             \n$value: math.random($limit: 10);\
             \na {b: $value > 0 and $value <= 10}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn no_arg() {
    let runner = runner().with_cwd("no_arg");
    assert_eq!(
        runner.ok("@use \"sass:math\";\
             \n$value: math.random();\
             \na {b: $value >= 0 and $value < 1}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn null() {
    let runner = runner().with_cwd("null");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"../util\";\
             \n@function check($value) {@return $value >= 0 and $value < 1}\
             \n@include util.check-values(null, meta.get-function(check));\n"
        ),
        ""
    );
}
#[test]
fn one() {
    let runner = runner().with_cwd("one");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"../util\";\
             \n@function check($value) {@return $value == 1}\
             \n@include util.check-values(1, meta.get-function(check));\n"),
        ""
    );
}
#[test]
fn one_hundred() {
    let runner = runner().with_cwd("one_hundred");
    assert_eq!(
        runner.ok(
            "@use \"sass:math\";\
             \n@use \"sass:meta\";\
             \n@use \"../util\";\
             \n@function check($value) {@return $value == math.round($value) and $value > 0 and $value <= 100}\
             \n@include util.check-values(100, meta.get-function(check));\n"
        ),
        ""
    );
}
#[test]
fn two() {
    let runner = runner().with_cwd("two");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"../util\";\
             \n@function check($value) {@return $value == 1 or $value == 2}\
             \n@include util.check-values(2, meta.get-function(check));\n"),
        ""
    );
}
#[test]
fn within_precision() {
    let runner = runner().with_cwd("within_precision");
    assert_eq!(
        runner.ok(
            "@use \"sass:math\";\
             \n// This is within the precision limit to be considered identical to 1.\
             \na {b: math.random(1.0000000000001)}\n"
        ),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
