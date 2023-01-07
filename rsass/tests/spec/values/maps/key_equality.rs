//! Tests auto-converted from "sass-spec/spec/values/maps/key_equality.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("key_equality")
}

mod infinity {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("a {b: inspect(map-get(((-1/0): b), -1/0))}\n"),
            "a {\
         \n  b: b;\
         \n}\n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            runner().ok("a {b: inspect(map-get(((1/0): b), 1/0))}\n"),
            "a {\
         \n  b: b;\
         \n}\n"
        );
    }
}
#[test]
fn nan() {
    assert_eq!(
        runner().ok("a {b: inspect(map-get(((0/0): b), 0/0))}\n"),
        "a {\
         \n  b: null;\
         \n}\n"
    );
}
