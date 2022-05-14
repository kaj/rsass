//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/186_test_newlines_removed_from_selectors_when_compressed.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("186_test_newlines_removed_from_selectors_when_compressed")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a\
             \n, b {\
             \n  z & {\
             \n    display: block;\
             \n  }\
             \n}\n"),
        "z a,\
         \nz b {\
         \n  display: block;\
         \n}\n"
    );
}
