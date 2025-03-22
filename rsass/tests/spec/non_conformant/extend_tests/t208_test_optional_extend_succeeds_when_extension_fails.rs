//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/208_test_optional_extend_succeeds_when_extension_fails.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("208_test_optional_extend_succeeds_when_extension_fails")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a.bar {a: b}\
             \nb.foo {@extend .bar !optional}\n"),
        "a.bar {\
         \n  a: b;\
         \n}\n"
    );
}
