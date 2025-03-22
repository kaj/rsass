//! Tests auto-converted from "sass-spec/spec/non_conformant/basic"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic")
}

mod t00_empty;

mod t01_simple_css;

mod t02_simple_nesting;

mod t03_simple_variable;

mod t04_basic_variables;

mod t05_empty_levels;

mod t06_nesting_and_comments;

mod t07_nested_simple_selector_groups;

mod t08_selector_combinators;

mod t09_selector_groups_and_combinators;

mod t10_classes_and_ids;

mod t11_attribute_selectors;

mod t12_pseudo_classes_and_elements;

mod t13_back_references;

mod t14_imports;

mod t15_arithmetic_and_lists;

mod t17_basic_mixins;

mod t18_mixin_scope;

mod t19_full_mixin_craziness;

mod t20_scoped_variables;

mod t21_one_builtin_function;

mod t22_colors_with_alpha;

mod t23_basic_value_interpolation;

mod t24_namespace_properties;

mod t25_basic_string_interpolation;

mod t26_selector_interpolation;

mod t27_media_queries;

mod t28_url;

mod t29_if;

mod t30_if_in_function;

mod t31_if_in_mixin;

mod t32_percentages;

mod t33_ambiguous_imports;

mod t35_varargs_false;

mod t36_extra_commas_in_selectors;

mod t37_url_expressions;

mod t38_expressions_in_at_directives;

mod t39_dash_match_attribute_selector;

mod t40_pseudo_class_identifier_starting_with_n;

mod t41_slashy_urls;

mod t42_css_imports;

mod t44_bem_selectors;

mod t49_interpolants_in_css_imports;

mod t50_wrapped_pseudo_selectors;

mod t51_trailing_commas_in_list;

mod t52_interchangeable_hyphens_underscores;

mod t53_escaped_quotes;

mod t54_adjacent_identifiers_with_hyphens;

mod t55_variable_exists;

mod t56_global_variable_exists;

mod t57_function_exists;

mod t58_mixin_exists;

mod t59_if_expression;
