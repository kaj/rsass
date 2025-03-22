//! Tests auto-converted from "sass-spec/spec/variables/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod after_colon {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("$a:\
             \n  b\n"),
            ""
        );
    }
}
mod before_colon {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("$a\
             \n  : b\n"),
            ""
        );
    }
}
mod before_default {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("$a: b\
             \n  !default;\n"),
            ""
        );
    }
}
mod before_global {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("$a: b\
             \n  !global;\n"),
            ""
        );
    }
}
mod between_double_default {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("$a: b !default\
             \n  !default;\n"),
            ""
        );
    }
}
