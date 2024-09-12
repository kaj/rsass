//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/inspect.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("inspect")
}

mod empty {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn bracketed() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.inspect([])}\n"),
            "a {\
         \n  b: [];\
         \n}\n"
        );
    }
}
