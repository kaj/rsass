//! Tests auto-converted from "sass-spec/spec/css/plain/hacks.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hacks").mock_file(
        "plain.css",
        ".hacks {\n  *x: y;\n  :x: y;\n  #x: y;\n  .x: y;\n}\n",
    )
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"plain\";\n"),
        ".hacks {\
         \n  *x: y;\
         \n  :x: y;\
         \n  #x: y;\
         \n  .x: y;\
         \n}\n"
    );
}
