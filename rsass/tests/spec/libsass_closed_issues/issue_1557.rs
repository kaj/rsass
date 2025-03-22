//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1557.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1557")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$xs-break: 30em;@media ALL AND (max-width: $xs-break) {header {display: none;}}\n"
        ),
        "@media ALL and (max-width: 30em) {\
         \n  header {\
         \n    display: none;\
         \n  }\
         \n}\n"
    );
}
