//! Tests auto-converted from "sass-spec/spec/core_functions/string/slice/unquoted.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unquoted")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.slice(cdefgh, 3, 5)}\n"),
        "a {\
         \n  b: efg;\
         \n}\n"
    );
}
