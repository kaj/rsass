//! Tests auto-converted from "sass-spec/spec/core_functions/selector/nest/many_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("many_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"d\", \"e\", \"f\", \"g\")}\n"),
        "a {\
         \n  b: c d e f g;\
         \n}\n"
    );
}
