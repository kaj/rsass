//! Tests auto-converted from "sass-spec/spec/non_conformant/sass/imported.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file(
        "imported.sass",
        "div\n\ta\n\t\tcolor: red\n\tli\n\t\tcolor: green",
    )
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@import \"imported.sass\";\n"),
        "div a {\
         \n  color: red;\
         \n}\
         \ndiv li {\
         \n  color: green;\
         \n}\n"
    );
}
