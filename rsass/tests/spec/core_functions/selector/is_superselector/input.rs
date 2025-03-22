//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/input.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("input")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \n// The full set of possible input formats is tested with `selector-parse()`;\
             \n// this spec just verifies one example for `is-superselector()`.\
             \na {b: selector.is-superselector((c, d e), (c, d e))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
