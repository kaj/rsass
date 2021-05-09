//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_73.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin box-shadow($shadow...) { \r\
             \n  -webkit-box-shadow: $shadow;\r\
             \n     -moz-box-shadow: $shadow;\r\
             \n          box-shadow: $shadow;\r\
             \n}"),
        ""
    );
}
