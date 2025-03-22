//! Tests auto-converted from "sass-spec/spec/core_functions/selector/nest/combinator.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("combinator")
}

mod leading {
    use super::runner;

    #[test]
    fn test_final() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"+ d\")}\n"),
            "a {\
         \n  b: c + d;\
         \n}\n"
        );
    }
    #[test]
    fn initial() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"> c\", \"d\")}\n"),
            "a {\
         \n  b: > c d;\
         \n}\n"
        );
    }
}
mod multiple {
    use super::runner;

    mod leading {
        use super::runner;

        #[test]
        fn test_final() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"+ > d\")}\n"),
                "a {\
         \n  b: c + > d;\
         \n}\n"
            );
        }
        #[test]
        fn initial() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"~ ~ c\", \"d\")}\n"),
                "a {\
         \n  b: ~ ~ c d;\
         \n}\n"
            );
        }
    }
    #[test]
    fn middle() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c > > d\", \"e\")}\n"),
            "a {\
         \n  b: c > > d e;\
         \n}\n"
        );
    }
    mod trailing {
        use super::runner;

        #[test]
        fn test_final() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"d + >\")}\n"),
                "a {\
         \n  b: c d + >;\
         \n}\n"
            );
        }
        #[test]
        fn initial() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c > ~\", \"d\")}\n"),
                "a {\
         \n  b: c > ~ d;\
         \n}\n"
            );
        }
    }
}
mod only {
    use super::runner;

    #[test]
    fn after() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"+\")}\n"),
            "a {\
         \n  b: c +;\
         \n}\n"
        );
    }
    #[test]
    fn before() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"~\", \"c\")}\n"),
            "a {\
         \n  b: ~ c;\
         \n}\n"
        );
    }
    #[test]
    fn between() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \">\", \"d\")}\n"),
            "a {\
         \n  b: c > d;\
         \n}\n"
        );
    }
}
mod trailing {
    use super::runner;

    #[test]
    fn test_final() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"d >\")}\n"),
            "a {\
         \n  b: c d >;\
         \n}\n"
        );
    }
    #[test]
    fn initial() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c ~\", \"d\")}\n"),
            "a {\
         \n  b: c ~ d;\
         \n}\n"
        );
    }
}
