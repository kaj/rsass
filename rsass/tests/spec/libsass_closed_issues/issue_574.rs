//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_574.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_574")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \n$flow: left;\n\
             \n$map: (\
             \n  margin-#{$flow}: 3em,\
             \n  foo: bar,\
             \n);\n\
             \n.test {\
             \n  margin-left: map.get($map, margin-left);\
             \n}\n"),
        ".test {\
         \n  margin-left: 3em;\
         \n}\n"
    );
}
