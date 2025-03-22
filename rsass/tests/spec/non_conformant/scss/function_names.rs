//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/function-names.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function-names")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  color: unquote(\"hello\");\
             \n  color: un#{quo}te(\"hello\");\
             \n  color: (\"hello\")un#{quo}te;\
             \n}\n"),
        "div {\
         \n  color: hello;\
         \n  color: unquote(\"hello\");\
         \n  color: \"hello\" unquote;\
         \n}\n"
    );
}
