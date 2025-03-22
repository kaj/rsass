//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_652.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_652")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \n$map: (\
             \n    purple: foo,\
             \n    rgba(1,2,3,1): bar,\
             \n    #ffffff: baz,\
             \n);\n\
             \na {\
             \n  name: map.get($map, purple) == foo;\
             \n  func: map.get($map, rgba(1,2,3,1)) == bar;\
             \n  hex: map.get($map, #ffffff) == baz;\
             \n}\n"),
        "a {\
         \n  name: true;\
         \n  func: true;\
         \n  hex: true;\
         \n}\n"
    );
}
