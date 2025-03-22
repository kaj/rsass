//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_992.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_992")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$color: \'red\';\n\
             \n.-text-#{$color}- {\
             \n  color: $color;\
             \n}"),
        ".-text-red- {\
         \n  color: \"red\";\
         \n}\n"
    );
}
