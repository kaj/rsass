//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/miss/simple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@import \"hey1.css\", \"cookie.css\", url(\"hey2.css\"), \"fudge.css\";\n\
             \n$foo:\"goodbye\";\
             \ndiv[name=\"hello\"] {\
             \n  color: blue;\
             \n}\n\
             \n@import \"bludge.css\";"
        ),
        "@import \"hey1.css\";\
         \n@import \"cookie.css\";\
         \n@import url(\"hey2.css\");\
         \n@import \"fudge.css\";\
         \n@import \"bludge.css\";\
         \ndiv[name=hello] {\
         \n  color: blue;\
         \n}\n"
    );
}
