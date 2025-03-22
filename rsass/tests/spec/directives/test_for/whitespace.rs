//! Tests auto-converted from "sass-spec/spec/directives/for/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod after_from {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@for $i from \
             \n  1 through 10 {}\n"),
            ""
        );
    }
}
mod after_through {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@for $i from 1 through\
             \n  10 {}\n\n"),
            ""
        );
    }
}
mod after_to {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@for $i from 1 to\
             \n  10 {}\n\n"),
            ""
        );
    }
}
mod before_block {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@for $i from 1 through 10 \
             \n  {}\n"),
            ""
        );
    }
}
mod before_from {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@for $i\
             \n  from 1 through 10 {}\n"),
            ""
        );
    }
}
mod before_through {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@for $i from 1\
             \n  through 10 {}\n"),
            ""
        );
    }
}
mod before_to {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@for $i from 1\
             \n  to 10 {}\n"),
            ""
        );
    }
}
mod before_var {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@for \
             \n  $i from 1 through 10 {}\n"),
            ""
        );
    }
}
