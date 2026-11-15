//! Tests auto-converted from "sass-spec/spec/values/numbers/units/escaped.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("escaped")
}

mod escape {
    use super::runner;

    #[test]
    fn normalized() {
        assert_eq!(
            runner().ok("a {b: 1\\65 m == 1em}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn preserved() {
        assert_eq!(
            runner().ok("a {b: 1\\% == 1%}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
