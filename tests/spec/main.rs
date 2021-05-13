//! Tests auto-converted from "sass-spec/spec"
//! version 630fd215, 2021-05-12 14:19:16 -0700.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["core_functions/selector/extend", "core_functions/selector/is_superselector", "core_functions/selector/unify", "directives/extend", "directives/forward", "libsass-closed-issues/issue_185/mixin.hrx", "libsass-closed-issues/issue_1801", "libsass-todo-issues/issue_1801", "libsass-todo-issues/issue_221262.hrx", "libsass-todo-issues/issue_221260.hrx", "libsass-todo-issues/issue_221292.hrx", "libsass/unicode-bom/utf-16-big", "libsass/unicode-bom/utf-16-little", "non_conformant/scss/huge.hrx", "non_conformant/scss/mixin-content.hrx", "non_conformant/scss/multiline_var.hrx"]
mod testrunner;
use testrunner::{runner, TestRunner};

mod arguments;

mod core_functions;

mod css;

mod directives;

mod expressions;

mod libsass;

mod libsass_closed_issues;

mod libsass_todo_issues;

mod libsass_todo_tests;

mod non_conformant;

mod parser;

mod values;
