//! Tests auto-converted from "sass-spec/spec/core_functions/string/slice/named.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \na {b: string.slice($string: \"cde\", $start-at: 2, $end-at: 2)}\n"
        ),
        "a {\
         \n  b: \"d\";\
         \n}\n"
    );
}
