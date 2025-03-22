//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1672.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1672")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$breakpoint: \'tablet\';\n\
             \n.-#{$breakpoint} {\
             \n  color: #FFF;\
             \n}"),
        ".-tablet {\
         \n  color: #FFF;\
         \n}\n"
    );
}
