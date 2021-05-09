//! Tests auto-converted from "sass-spec/spec/core_functions/list/utils.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod empty_map {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn same_as_empty_list() {
        assert_eq!(
            runner().ok("@import \"core_functions/list/utils\";\
             \na {b: $empty-map == ()}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
mod real_separator {
    #[allow(unused)]
    use super::runner;
    mod empty {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: real-separator($empty-comma-list)}\n"),
                "a {\
         \n  b: comma;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: real-separator($empty-space-list)}\n"),
                "a {\
         \n  b: space;\
         \n}\n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: real-separator(())}\n"),
                "a {\
         \n  b: undecided;\
         \n}\n"
            );
        }
    }
    mod multi {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: real-separator((1, 2))}\n"),
                "a {\
         \n  b: comma;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: real-separator(1 2)}\n"),
                "a {\
         \n  b: space;\
         \n}\n"
            );
        }
    }
    mod single {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: real-separator((1,))}\n"),
                "a {\
         \n  b: comma;\
         \n}\n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: real-separator([1])}\n"),
                "a {\
         \n  b: undecided;\
         \n}\n"
            );
        }
    }
}
mod with_separator {
    #[allow(unused)]
    use super::runner;
    mod multi {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: with-separator(1 2, comma)}\n"),
                "a {\
         \n  b: 1, 2;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: with-separator((1, 2), space)}\n"),
                "a {\
         \n  b: 1 2;\
         \n}\n"
            );
        }
    }
    mod single {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: real-separator(with-separator([1], comma))}\n"),
                "a {\
         \n  b: comma;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: real-separator(with-separator((1,), space))}\n"),
                "a {\
         \n  b: space;\
         \n}\n"
            );
        }
    }
}
