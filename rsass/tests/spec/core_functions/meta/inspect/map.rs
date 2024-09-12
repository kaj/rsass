//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/map.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("map")
}

mod list {
    #[allow(unused)]
    use super::runner;

    mod key {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect(((1, 2): 3, (4, 5): 6));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: ((1, 2): 3, (4, 5): 6);\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect((1 2: 3, 4 5: 6));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1 2: 3, 4 5: 6);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod value {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect((1: (2, 3), 4: (5, 6)));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1: (2, 3), 4: (5, 6));\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect((1: 2 3, 4: 5 6));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1: 2 3, 4: 5 6);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
}
#[test]
fn number() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect((1: 2, 3: 4));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
        "a {\
         \n  value: (1: 2, 3: 4);\
         \n  type: string;\
         \n}\n"
    );
}
