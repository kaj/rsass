//! Tests auto-converted from "sass-spec/spec/values/lists/equality.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("equality")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  @if [foo bar]==[foo bar] {\
             \n    t1: t;\
             \n  } @else {\
             \n    f1: f;\
             \n  }\n\
             \n  @if [foo bar]==[foo, bar] {\
             \n    t2: t;\
             \n  } @else {\
             \n    f2: f;\
             \n  }\n\
             \n  @if [foo bar]==(foo bar) {\
             \n    t3: t;\
             \n  } @else {\
             \n    f3: f;\
             \n  }\n\
             \n  @if [] == [] {\
             \n    t4: t;\
             \n  } @else {\
             \n    f4: f;\
             \n  }\n\
             \n  @if [] == () {\
             \n    t5: t;\
             \n  } @else {\
             \n    f5: f;\
             \n  }\
             \n}\n"),
        "a {\
         \n  t1: t;\
         \n  f2: f;\
         \n  f3: f;\
         \n  t4: t;\
         \n  f5: f;\
         \n}\n"
    );
}
