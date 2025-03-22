//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/interpolated-selectors.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolated-selectors")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo#{bar} hux {\
             \n  color: red;\
             \n}"),
        "foobar hux {\
         \n  color: red;\
         \n}\n"
    );
}
