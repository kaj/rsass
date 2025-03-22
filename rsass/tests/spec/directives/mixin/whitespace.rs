//! Tests auto-converted from "sass-spec/spec/directives/mixin/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod include {
    use super::runner;

    mod after_using {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@mixin a {@content}\
             \n@include a() using\
             \n  () {}\n"),
                ""
            );
        }
    }
    mod after_using_arglist {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@mixin a {@content}\
             \n@include a() using ()\
             \n  {}\n"),
                ""
            );
        }
    }
    mod before_block {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@mixin a {@content}\
             \n@include a()\
             \n  {}\n"),
                ""
            );
        }
    }
    mod before_name {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@mixin a {}\
             \n@include\
             \n  a\n"),
                ""
            );
        }
    }
    mod before_using {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@mixin a {@content}\
             \n@include a()\
             \n  using () {}\n"),
                ""
            );
        }
    }
}
mod mixin {
    use super::runner;

    mod after_args {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@mixin a()\
             \n  {}\n"),
                ""
            );
        }
    }
    mod before_name {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@mixin\
             \n  a {}\n"),
                ""
            );
        }
    }
}
