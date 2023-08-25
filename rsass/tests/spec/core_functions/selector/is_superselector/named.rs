//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/named.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
fn test() {
    assert_eq!(
        runner()
            .ok("a {b: is-superselector($super: \"c\", $sub: \"c.d\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
