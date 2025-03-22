//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/string.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("string")
}

#[test]
fn quoted() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n$result: meta.inspect(\"foo\");\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\n\
             \n  // inspect() should always return an unquoted string, so when it\'s passed a\
             \n  // quoted string its return value should contain quote characters. We check\
             \n  // the length to verify that the quotes are included, since there\'s no\
             \n  // built-in way to check whether a string is quoted.\
             \n  length: string.length($result);\
             \n}\n"
        ),
        "a {\
         \n  value: \"foo\";\
         \n  type: string;\
         \n  length: 5;\
         \n}\n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect(foo);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
        "a {\
         \n  value: foo;\
         \n  type: string;\
         \n}\n"
    );
}
