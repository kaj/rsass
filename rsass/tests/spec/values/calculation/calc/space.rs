//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/space.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("space")
}

mod interpolation {
    use super::runner;

    #[test]
    fn after() {
        assert_eq!(
            runner().ok("a {b: calc(1 #{\"+ 2\"})}\n"),
            "a {\
         \n  b: calc(1 + 2);\
         \n}\n"
        );
    }
    #[test]
    fn before() {
        assert_eq!(
            runner().ok("a {b: calc(#{\"1 +\"} 2)}\n"),
            "a {\
         \n  b: calc(1 + 2);\
         \n}\n"
        );
    }
    #[test]
    fn between() {
        assert_eq!(
            runner().ok("a {b: calc(1 #{\"+ 2 +\"} 3)}\n"),
            "a {\
         \n  b: calc(1 + 2 + 3);\
         \n}\n"
        );
    }
}
mod var {
    use super::runner;

    #[test]
    fn after() {
        assert_eq!(
            runner().ok("a {b: calc(1 var(--c))}\n"),
            "a {\
         \n  b: calc(1 var(--c));\
         \n}\n"
        );
    }
    #[test]
    fn before() {
        assert_eq!(
            runner().ok("a {b: calc(var(--c) 1)}\n"),
            "a {\
         \n  b: calc(var(--c) 1);\
         \n}\n"
        );
    }
    #[test]
    fn between() {
        assert_eq!(
            runner().ok("a {b: calc(1 var(--c) 2)}\n"),
            "a {\
         \n  b: calc(1 var(--c) 2);\
         \n}\n"
        );
    }
}
mod variable {
    use super::runner;

    #[test]
    fn after() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \n$c: string.unquote(\"+ 2\");\
             \na {b: calc(1 $c)}\n"),
            "a {\
         \n  b: calc(1 + 2);\
         \n}\n"
        );
    }
    #[test]
    fn before() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \n$c: string.unquote(\"1 +\");\
             \na {b: calc($c 2)}\n"),
            "a {\
         \n  b: calc(1 + 2);\
         \n}\n"
        );
    }
    #[test]
    fn between() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \n$c: string.unquote(\"+ 2 +\");\
             \na {b: calc(1 $c 3)}\n"),
            "a {\
         \n  b: calc(1 + 2 + 3);\
         \n}\n"
        );
    }
}
