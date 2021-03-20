//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_748.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "// problem: not expression is currently returning false on values other than true, false or null\
            \n\
            \n@function truthyfalsey($bool: null) {\
            \n  @if not $bool {\
            \n    @return falsey;\
            \n  } @else {\
            \n    @return truthy;\
            \n  }\
            \n}\
            \n\
            \n.test {\
            \n  debug: truthyfalsey(true); // expect truthy\
            \n  debug: truthyfalsey(false); // expect falsey\
            \n  debug: truthyfalsey(); // expect falsey (default arg is null)\
            \n  debug: truthyfalsey(5); // expect truthy\
            \n  debug: truthyfalsey(string); // expect truthy\
            \n  debug: truthyfalsey((alpha: 1, bravo: 2)); // expect truthy\
            \n  debug: truthyfalsey(this is a list); // expect truthy\
            \n  debug: truthyfalsey(\'true\'); // expect truthy\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  debug: truthy;\
        \n  debug: falsey;\
        \n  debug: falsey;\
        \n  debug: truthy;\
        \n  debug: truthy;\
        \n  debug: truthy;\
        \n  debug: truthy;\
        \n  debug: truthy;\
        \n}\
        \n"
    );
}
