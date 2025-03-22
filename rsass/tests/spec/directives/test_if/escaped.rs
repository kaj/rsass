//! Tests auto-converted from "sass-spec/spec/directives/if/escaped.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("escaped")
}

#[test]
fn if_only() {
    assert_eq!(
        runner().ok(
            "// Escapes should be normalized before directives are parsed.\
             \n@\\69 f true {a {b: c}}\n"
        ),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn with_else() {
    assert_eq!(
        runner().ok("// See sass/dart-sass#1011\
             \n@if false {}\
             \n@\\65lse {a {b: c}}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
