//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_992.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
