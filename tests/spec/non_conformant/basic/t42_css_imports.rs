//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/42_css_imports.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  color: red;\
             \n}\n\
             \n@import \"hux\\ bux.css\";\
             \n@import \"foo.css\";\n\
             \nspan {\
             \n  color: blue;\
             \n}\n\
             \n@import \"bar.css\";"),
        "@import \"hux\\ bux.css\";\
         \n@import \"foo.css\";\
         \n@import \"bar.css\";\
         \ndiv {\
         \n  color: red;\
         \n}\
         \nspan {\
         \n  color: blue;\
         \n}\n"
    );
}
