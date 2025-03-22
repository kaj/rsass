//! Tests auto-converted from "sass-spec/spec/css/unknown_directive/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod children {
    use super::runner;

    mod before_value {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@a\
             \n  b {}\n"),
                "@a b {}\n"
            );
        }
    }
    mod no_value {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@a\
             \n  {}\n"),
                "@a {}\n"
            );
        }
    }
}
mod no_children {
    use super::runner;

    mod before_value {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@a \
             \n  b\n"),
                "@a b;\n"
            );
        }
    }
}
