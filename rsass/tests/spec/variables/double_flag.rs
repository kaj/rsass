//! Tests auto-converted from "sass-spec/spec/variables/double_flag.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("double_flag")
}

#[test]
fn default() {
    assert_eq!(
        runner().ok("$a: b !default !default;\n\
             \nc {d: $a}\n"),
        "c {\
         \n  d: b;\
         \n}\n"
    );
}
#[test]
fn global() {
    assert_eq!(
        runner().ok("$a: b;\
             \nc {\
             \n  $a: d !global !global;\
             \n  e: $a\
             \n}\n"),
        "c {\
         \n  e: d;\
         \n}\n"
    );
}
