//! Tests auto-converted from "sass-spec/spec"
//! version 90b5e98f6, 2025-08-20 16:30:28 -0700.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["directives/extend", "libsass-closed-issues/issue_2446", "libsass-closed-issues/issue_245443", "libsass-todo-issues/issue_221260.hrx", "libsass-todo-issues/issue_221262.hrx", "libsass-todo-issues/issue_221264.hrx", "libsass-todo-issues/issue_221267", "libsass-todo-issues/issue_221286", "libsass-todo-issues/issue_221286", "libsass-todo-issues/issue_221292.hrx", "libsass-todo-issues/issue_245442", "libsass-todo-issues/issue_245446", "libsass-todo-tests/errors/unicode", "libsass/unicode-bom/utf-16-big", "libsass/unicode-bom/utf-16-little", "non_conformant/sass", "non_conformant/scss/huge.hrx", "non_conformant/scss/multiline-var.hrx"]
mod testrunner;
use testrunner::{runner, TestRunner};

mod callable;

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
