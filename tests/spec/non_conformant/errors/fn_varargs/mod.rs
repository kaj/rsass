//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-varargs"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/errors/fn-varargs/at-start.hrx"

// Ignoring "at_start", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/errors/fn-varargs/multiple.hrx"

// Ignoring "multiple", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/errors/fn-varargs/with-default.hrx"

// Ignoring "with_default", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/errors/fn-varargs/with-optional.hrx"
#[test]
fn with_optional() {
    assert_eq!(
        rsass("@function test($param:\"default\",$rest...) {}").unwrap(),
        ""
    );
}
