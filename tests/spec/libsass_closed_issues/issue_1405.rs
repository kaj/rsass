//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1405.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\r\
            \n  foo: (1a2b3c);\r\
            \n\r\
            \n  length-1: length(1a2b3c);\r\
            \n\r\
            \n  unit-1: unit(1a2b3c);\r\
            \n\r\
            \n  result-1: 1em-.75em;\r\
            \n  result-2: 2em-1em;\r\
            \n  result-3: 2em-0.75em;\r\
            \n  result-4: 1.5em-1em;\r\
            \n  result-5: 2em-1.5em;\r\
            \n\r\
            \n  type-1: type-of(1em-.75em);\r\
            \n  type-2: type-of(2em-1em);\r\
            \n  type-3: type-of(2em-0.75em);\r\
            \n  type-4: type-of(1.5em-1em);\r\
            \n  type-5: type-of(2em-1.5em);\r\
            \n  type-6: type-of(1a2b3c);\r\
            \n\r\
            \n  test-1: (1-em-2-em);\r\
            \n  test-1: (1-em - 2-em);\r\
            \n\r\
            \n  test-2: (1-0-em-2-0-em);\r\
            \n  test-2: (1-0-em - 2-0-em);\r\
            \n\r\
            \n  test-3: (1-A-em-2-A-em);\r\
            \n  test-3: (1-A-em - 2-A-em);\r\
            \n\r\
            \n  test-4: (1_em--_--e-2_em--_--e);\r\
            \n  test-4: (1_em--_--e - 2_em--_--e);\r\
            \n\r\
            \n  test-5: (1_em--_--e0-2_em--_--e0);\r\
            \n  test-5: (1_em--_--e0 - 2_em--_--e0);\r\
            \n\r\
            \n  test-6: (1_em--_--e0__-2_em--_--e0__);\r\
            \n  test-6: (1_em--_--e0__ - 2_em--_--e0__);\r\
            \n\r\
            \n  test-7: (1\\65 _em--_--e0-2\\65 _em--_--e0);\r\
            \n  test-7: (1\\65 _em--_--e0 - 2\\65 _em--_--e0);\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: 1a2b3c;\
        \n  length-1: 1;\
        \n  unit-1: \"a2b3c\";\
        \n  result-1: 0.25em;\
        \n  result-2: 1em;\
        \n  result-3: 1.25em;\
        \n  result-4: 0.5em;\
        \n  result-5: 0.5em;\
        \n  type-1: number;\
        \n  type-2: number;\
        \n  type-3: number;\
        \n  type-4: number;\
        \n  type-5: number;\
        \n  type-6: number;\
        \n  test-1: -1-em;\
        \n  test-1: -1-em;\
        \n  test-2: -1-em;\
        \n  test-2: -1-em;\
        \n  test-3: -1-A-em;\
        \n  test-3: -1-A-em;\
        \n  test-4: -1_em--_--e;\
        \n  test-4: -1_em--_--e;\
        \n  test-5: -1_em--_--e0;\
        \n  test-5: -1_em--_--e0;\
        \n  test-6: -1_em--_--e0__;\
        \n  test-6: -1_em--_--e0__;\
        \n  test-7: -1e_em--_--e0;\
        \n  test-7: -1e_em--_--e0;\
        \n}\
        \n"
    );
}
