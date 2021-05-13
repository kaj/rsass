//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_698.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "test {\
             \n  test: foo + null;\
             \n}\n"
        ),
        "Error: Invalid null operation: \"\"foo\" plus null\".\
         \n        on line 2 of input.scss\
         \n  Use --trace for backtrace.",
    );
}
