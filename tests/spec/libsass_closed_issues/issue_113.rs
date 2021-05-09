//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_113.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("// Input\
             \nsection {\
             \n    $w: null, 10px;\
             \n    width: $w;\
             \n}"),
        "section {\
         \n  width: 10px;\
         \n}\n"
    );
}
