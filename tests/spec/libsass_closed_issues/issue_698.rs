//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_698.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\
             \n  test: foo + null;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Invalid null operation: \"\"foo\" plus null\".\
         \n        on line 2 of input.scss\
         \n  Use --trace for backtrace.\
         \n",
    );
}
