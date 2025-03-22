//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod content;

mod from_other;

mod load;

mod member;

mod too_few_args;

mod too_many_args;

mod test_type;

mod with;
