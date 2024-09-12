//! Tests auto-converted from "sass-spec/spec/core_functions/string/slice/end.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("end")
}

mod negative {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn t1() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, -1)}\n"),
            "a {\
         \n  b: \"cde\";\
         \n}\n"
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, -2)}\n"),
            "a {\
         \n  b: \"cd\";\
         \n}\n"
        );
    }
    #[test]
    fn after_last() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, -100)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, -4)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
}
mod positive {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn t0() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, 0)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
    #[test]
    fn t1() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, 1)}\n"),
            "a {\
         \n  b: \"c\";\
         \n}\n"
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, 2)}\n"),
            "a {\
         \n  b: \"cd\";\
         \n}\n"
        );
    }
    #[test]
    fn after_last() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, 100)}\n"),
            "a {\
         \n  b: \"cde\";\
         \n}\n"
        );
    }
    #[test]
    fn after_start() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"cdef\", 2, 3)}\n"),
            "a {\
         \n  b: \"de\";\
         \n}\n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            runner().ok("@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, 3)}\n"),
            "a {\
         \n  b: \"cde\";\
         \n}\n"
        );
    }
}
