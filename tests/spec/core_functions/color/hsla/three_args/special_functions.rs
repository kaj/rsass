//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/three_args/special_functions.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  calc-1: hsla(calc(1), 2%, 3%);\
            \n  calc-2: hsla(1, calc(2%), 3%);\
            \n  calc-3: hsla(1, 2%, calc(3%));\
            \n\
            \n  var-1: hsla(var(--foo), 2%, 3%);\
            \n  var-2: hsla(1, var(--foo), 3%);\
            \n  var-3: hsla(1, 2%, var(--foo));\
            \n\
            \n  env-1: hsla(env(--foo), 2%, 3%);\
            \n  env-2: hsla(1, env(--foo), 3%);\
            \n  env-3: hsla(1, 2%, env(--foo));\
            \n\
            \n  min-1: hsla(min(1), 2%, 3%);\
            \n  min-2: hsla(1, min(2%), 3%);\
            \n  min-3: hsla(1, 2%, min(3%));\
            \n\
            \n  max-1: hsla(max(1), 2%, 3%);\
            \n  max-2: hsla(1, max(2%), 3%);\
            \n  max-3: hsla(1, 2%, max(3%));\
            \n\
            \n  clamp-1: hsla(clamp(1, 2, 3), 2%, 3%);\
            \n  clamp-2: hsla(1, clamp(2%, 3%, 4%), 3%);\
            \n  clamp-3: hsla(1, 2%, clamp(3%, 4%, 5%));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  calc-1: hsla(calc(1), 2%, 3%);\
        \n  calc-2: hsla(1, calc(2%), 3%);\
        \n  calc-3: hsla(1, 2%, calc(3%));\
        \n  var-1: hsla(var(--foo), 2%, 3%);\
        \n  var-2: hsla(1, var(--foo), 3%);\
        \n  var-3: hsla(1, 2%, var(--foo));\
        \n  env-1: hsla(env(--foo), 2%, 3%);\
        \n  env-2: hsla(1, env(--foo), 3%);\
        \n  env-3: hsla(1, 2%, env(--foo));\
        \n  min-1: hsla(min(1), 2%, 3%);\
        \n  min-2: hsla(1, min(2%), 3%);\
        \n  min-3: hsla(1, 2%, min(3%));\
        \n  max-1: hsla(max(1), 2%, 3%);\
        \n  max-2: hsla(1, max(2%), 3%);\
        \n  max-3: hsla(1, 2%, max(3%));\
        \n  clamp-1: hsla(clamp(1, 2, 3), 2%, 3%);\
        \n  clamp-2: hsla(1, clamp(2%, 3%, 4%), 3%);\
        \n  clamp-3: hsla(1, 2%, clamp(3%, 4%, 5%));\
        \n}\
        \n"
    );
}
