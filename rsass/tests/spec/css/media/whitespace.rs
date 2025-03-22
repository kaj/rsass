//! Tests auto-converted from "sass-spec/spec/css/media/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod after_query {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@media screen \
             \n  {}\n"),
            ""
        );
    }
}
mod before_query {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@media \
             \n  screen {}\n"),
            ""
        );
    }
}
