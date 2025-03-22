//! Tests auto-converted from "sass-spec/spec/values/maps/key_equality.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("key_equality")
}

mod infinity {
    use super::runner;

    #[test]
    fn negative() {
        assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n@use \"sass:math\";\
             \n@use \"sass:meta\";\n\
             \na {b: meta.inspect(map.get(((math.div(-1, 0)): b), math.div(-1, 0)))}\n"
        ),
        "a {\
         \n  b: b;\
         \n}\n"
    );
    }
    #[test]
    fn positive() {
        assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n@use \"sass:math\";\
             \n@use \"sass:meta\";\n\
             \na {b: meta.inspect(map.get(((math.div(1, 0)): b), math.div(1, 0)))}\n"
        ),
        "a {\
         \n  b: b;\
         \n}\n"
    );
    }
}
#[test]
fn nan() {
    assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n@use \"sass:math\";\
             \n@use \"sass:meta\";\n\
             \na {b: meta.inspect(map.get((math.div(0, 0): b), math.div(0, 0)))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
}
