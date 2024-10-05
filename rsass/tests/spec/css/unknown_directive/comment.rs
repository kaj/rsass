//! Tests auto-converted from "sass-spec/spec/css/unknown_directive/comment.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod children {
    #[allow(unused)]
    use super::runner;

    mod after_value {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@a b /**/ {}\n"), "@a b /**/ {}\n");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@a b //\
             \n  {}\n"),
                "@a b {}\n"
            );
        }
    }
    mod before_value {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@a /**/ b {}\n"), "@a b {}\n");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@a //\
             \n  b {}\n"),
                "@a b {}\n"
            );
        }
    }
    mod no_value {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@a /**/ {}\n"), "@a {}\n");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@a //\
             \n  {}\n"),
                "@a {}\n"
            );
        }
    }
}
mod no_children {
    #[allow(unused)]
    use super::runner;

    mod after_value {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@a b /**/\n"), "@a b /**/;\n");
        }
        #[test]
        fn silent() {
            assert_eq!(runner().ok("@a b //\n"), "@a b;\n");
        }
    }
    mod before_value {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@a /**/ b\n"), "@a b;\n");
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@a //\
             \n  b\n"),
                "@a b;\n"
            );
        }
    }
    mod no_value {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(runner().ok("@a /**/\n"), "@a;\n");
        }
        #[test]
        fn silent() {
            assert_eq!(runner().ok("@a //\n"), "@a;\n");
        }
    }
}
