//! Tests auto-converted from "sass-spec/spec/core_functions/string/slice/empty.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("empty")
}

mod end {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn t0() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"\", 1, 0)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
    #[test]
    fn t1() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"\", 1, 1)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"\", 1, 2)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
}
mod start {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn t0() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"\", 0)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
    #[test]
    fn t1() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"\", 1)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"\", 2)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
    #[test]
    fn negative_1() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"\", -1)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
}
