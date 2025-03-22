//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_864.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_864")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \ndiv { color: color.adjust(#999, $saturation: -50%); }"),
        "div {\
         \n  color: #999999;\
         \n}\n"
    );
}
