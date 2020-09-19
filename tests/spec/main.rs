//! Tests auto-converted from "sass-spec/spec"
//! version 45b9cbaf0, 2020-09-17 17:50:20 -0700.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["core_functions/selector/extend", "core_functions/selector/is_superselector", "core_functions/selector/unify", "directives/forward", "directives/use", "libsass-closed-issues/issue_185/mixin.hrx", "libsass-todo-issues/issue_221262.hrx", "libsass-todo-issues/issue_221292.hrx", "libsass/Sa\u{301}ss-UT\u{327}F8.hrx", "libsass/unicode-bom/utf-16-big", "libsass/unicode-bom/utf-16-little", "non_conformant/scss/huge.hrx", "non_conformant/scss/mixin-content.hrx", "non_conformant/scss/multiline_var.hrx"]
use rsass::{compile_scss, output::Format};

mod arguments;

mod core_functions;

mod css;

mod directives;

mod libsass;

mod libsass_closed_issues;

mod libsass_todo_issues;

mod libsass_todo_tests;

mod non_conformant;

mod values;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), Default::default())
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
#[allow(unused)]
fn rsass_fmt(format: Format, input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), format)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
