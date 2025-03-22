//! Tests auto-converted from "sass-spec/spec/core_functions/string/unquote.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unquote")
}

#[test]
fn empty() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n$result: string.unquote(\"\");\
             \na {\
             \n  result: $result; // This will not be emitted because the contents is empty.\
             \n  length: string.length($result);\
             \n  same: $result == \"\";\
             \n}\n"
        ),
        "a {\
         \n  length: 0;\
         \n  same: true;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.unquote()}\n"
            ),
            "Error: Missing argument $string.\
         \n  ,--> input.scss\
         \n2 | a {b: string.unquote()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function unquote($string) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.unquote(c, d)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: string.unquote(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function unquote($string) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.unquote(1)}\n"
            ),
            "Error: $string: 1 is not a string.\
         \n  ,\
         \n2 | a {b: string.unquote(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn escaped_backslash() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \n$result: string.unquote(\"\\\\0 \");\
             \na {\
             \n  result: $result;\
             \n  length: string.length($result);\
             \n  same-as-argument: $result == \"\\\\0 \";\
             \n  same-as-literal: $result == \\0 ;\
             \n}\n"),
        "a {\
         \n  result: \\0 ;\
         \n  length: 3;\
         \n  same-as-argument: true;\
         \n  same-as-literal: true;\
         \n}\n"
    );
}
mod escaped_quotes {
    use super::runner;

    #[test]
    fn quoted() {
        assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// Unquoting a quoted string returns an unquoted string with the same code\
             \n// points. Code points such as quotes that need to be escaped in the original\
             \n// don\'t need escaping in the output.\
             \n$result: string.unquote(\"\\\"c\\\"\");\
             \na {\
             \n  result: $result;\
             \n  length: string.length($result);\
             \n  same: $result == \"\\\"c\\\"\";\
             \n}\n"
        ),
        "a {\
         \n  result: \"c\";\
         \n  length: 3;\
         \n  same: true;\
         \n}\n"
    );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// Unquoting an unquoted string returns it exactly as-is, leaving escapes\
             \n// totally unchanged (whether they\'re quotes or not).\
             \n$result: string.unquote(\\\"c\\\");\
             \na {\
             \n  result: $result;\
             \n  length: string.length($result);\
             \n  same: $result == \\\"c\\\";\
             \n}\n"
        ),
        "a {\
         \n  result: \\\"c\\\";\
         \n  length: 5;\
         \n  same: true;\
         \n}\n"
    );
    }
}
#[test]
fn meaningful_css_characters() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// Unquoted strings aren\'t required to be valid CSS identifiers, and the\
             \n// `unquote()` function does *not* escape characters that aren\'t valid\
             \n// identifier characters. This allows it to be used as an escape hatch to\
             \n// produce CSS that Sass doesn\'t otherwise support.\
             \n$result: string.unquote(\"b; c {d: e\");\
             \na {\
             \n  result: $result;\
             \n  length: string.length($result);\
             \n  same: $result == \"b; c {d: e\";\
             \n}\n"
        ),
        "a {\
         \n  result: b; c {d: e;\
         \n  length: 10;\
         \n  same: true;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.unquote($string: c)}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn quoted() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.unquote(\"c\")}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.unquote(c)}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
