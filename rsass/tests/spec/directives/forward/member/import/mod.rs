//! Tests auto-converted from "sass-spec/spec/directives/forward/member/import"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("import")
}

mod forward_to_import;

mod import_to_forward;

mod precedence;
