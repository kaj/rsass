//! Tests auto-converted from "sass-spec/spec/non_conformant/scope"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("scope")
}

mod clash;

mod each;

mod test_for;

mod function;

mod test_if;

mod mixin;

mod nested;

mod root;

mod ruleset;

mod test_while;
