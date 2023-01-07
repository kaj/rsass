//! Tests auto-converted from "sass-spec/spec/libsass/css-import.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css-import")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \'foo.css\', \"bar.css\";\n\
             \ndiv {\
             \n  color: red;\
             \n}"),
        "@import \'foo.css\';\
         \n@import \"bar.css\";\
         \ndiv {\
         \n  color: red;\
         \n}\n"
    );
}
