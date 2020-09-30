//! Tests auto-converted from "sass-spec/spec/css/unknown_directive"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/css/unknown_directive/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;

    // Ignoring "in_declaration", error tests are not supported yet.

    // Ignoring "in_function", error tests are not supported yet.
    mod interpolation {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "in_declaration", error tests are not supported yet.

        // Ignoring "in_function", error tests are not supported yet.

        // Ignoring "space_after_at", error tests are not supported yet.
    }

    // Ignoring "space_after_at", error tests are not supported yet.
}

// From "sass-spec/spec/css/unknown_directive/name_interpolation.hrx"
#[test]
#[ignore] // unexepected error
fn name_interpolation() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/css/unknown_directive/plain.hrx"
#[test]
#[ignore] // unexepected error
fn plain() {
    assert_eq!(
        rsass(
            "// Unknown directives should support almost any sequence of valid tokens.\
            \n\
            \n// By default, characters are passed through unaltered.\
            \n@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\
            \n\
            \n// Strings are tokenized as strings.\
            \n@asdf \"f\'o\" \'b\"r\' url(baz) url(\"qux\");\
            \n\
            \n// Comments are preserved.\
            \n@asdf foo //\
            \n      bar;\
            \n@asdf foo /* bar */ baz;\
            \n"
        )
        .unwrap(),
        "@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\
        \n@asdf \"f\'o\" \'b\"r\' url(baz) url(\"qux\");\
        \n@asdf foo //\
        \n      bar;\
        \n@asdf foo /* bar */ baz;\
        \n"
    );
}

// From "sass-spec/spec/css/unknown_directive/value_interpolation.hrx"
#[test]
fn value_interpolation() {
    assert_eq!(
        rsass(
            "// Unknown directives should support interpolation in plain text, strings,\
            \n// identifiers, and URLs.\
            \n@asdf #{1 + 2};\
            \n\
            \n@asdf foo#{\"bar\"}baz;\
            \n\
            \n@asdf \"foo #{\"bar\"} baz\";\
            \n\
            \n@asdf \'foo #{\'bar\'} baz\';\
            \n\
            \n@asdf url(http://#{\")\"}.com/);\
            \n\
            \n@asdf url(\"http://#{\")\"}.com/\");\
            \n"
        )
        .unwrap(),
        "@asdf 3;\
        \n@asdf foobarbaz;\
        \n@asdf \"foo bar baz\";\
        \n@asdf \"foo bar baz\";\
        \n@asdf url(http://).com/);\
        \n@asdf url(\"http://).com/\");\
        \n"
    );
}
