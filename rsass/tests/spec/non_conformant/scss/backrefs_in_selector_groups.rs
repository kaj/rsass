//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/backrefs-in-selector-groups.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("backrefs-in-selector-groups")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  &:c, & d {\
             \n    hey: ho;\
             \n  }\
             \n}\n\
             \na b {\
             \n  &:c, & d {\
             \n    hey: ho;\
             \n  }\
             \n}\n"),
        "a:c, a d {\
         \n  hey: ho;\
         \n}\
         \na b:c, a b d {\
         \n  hey: ho;\
         \n}\n"
    );
}
