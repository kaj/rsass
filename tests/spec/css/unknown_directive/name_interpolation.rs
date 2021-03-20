//! Tests auto-converted from "sass-spec/spec/css/unknown_directive/name_interpolation.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// At-rule names may be interpolated. Any interpolated at-rule is treated as\
            \n// unknown, except for @keyframes which has behavior that\'s only triggered at\
            \n// eval-time.\
            \n\
            \n// Interpolation can appear anywhere in a rule.\
            \n@#{\"interpolated\"}-beginning;\
            \n@interopl#{\"ated-mid\"}dle;\
            \n@interoplated-#{\"end\"};\
            \n@#{\"entirely-interpolated\"};\
            \n\
            \n// An interpolated rule can have all the same kinds of values and blocks as any\
            \n// other unknown rule.\
            \n@#{\"plain\"} value;\
            \n@#{\"interpolated\"} #{\"value\"};\
            \n@#{\"block\"} {x: y}\
            \n@#{\"block-with\"} plain-value {x: y}\
            \n@#{\"block-with\"} #{\"interpolated\"}-value {x: y}\
            \n\
            \n// An interpolated at-rule doesn\'t count as a Sass at-rule.\
            \n@#{\"error\"} not really an error;\
            \n\
            \n// An interpolated at-rule doesn\'t have any special parse-time behavior, even if\
            \n// it ends up with the same name as an at-rule that does.\
            \n@#{\"media\"} ($var: value) {\
            \n  .x {y: z}\
            \n}\
            \n\
            \n// The @keyframes rule is an exception, because its special parse behavior is\
            \n// handled at runtime.\
            \n@#{\"keyframes\"} name {\
            \n  10% {x: y}\
            \n}\
            \n"
        )
        .unwrap(),
        "@interpolated-beginning;\
        \n@interoplated-middle;\
        \n@interoplated-end;\
        \n@entirely-interpolated;\
        \n@plain value;\
        \n@interpolated value;\
        \n@block {\
        \n  x: y;\
        \n}\
        \n@block-with plain-value {\
        \n  x: y;\
        \n}\
        \n@block-with interpolated-value {\
        \n  x: y;\
        \n}\
        \n@error not really an error;\
        \n@media ($var: value) {\
        \n  .x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@keyframes name {\
        \n  10% {\
        \n    x: y;\
        \n  }\
        \n}\
        \n"
    );
}
