//! Tests auto-converted from "sass-spec/spec"
//! version 7516e178c, 2023-10-31 15:54:57 -0700.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["core_functions/selector/extend", "directives/extend", "libsass-todo-issues/issue_221260.hrx", "libsass-todo-issues/issue_221262.hrx", "libsass-todo-issues/issue_221264.hrx", "libsass-todo-issues/issue_221292.hrx", "libsass/unicode-bom/utf-16-big", "libsass/unicode-bom/utf-16-little", "non_conformant/scss/huge.hrx", "non_conformant/scss/multiline-var.hrx"]
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

mod operators;

mod parser;

mod values;

mod variables;
