//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests"

#[allow(unused)]
use super::runner;

mod t001_test_basic;

mod t002_test_basic;

mod t003_test_basic;

mod t004_test_basic;

mod t005_test_multiple_targets;

mod t006_test_multiple_extendees;

mod t007_test_multiple_extends_with_single_extender_and_single_target;

mod t008_test_multiple_extends_with_single_extender_and_single_target;

mod t009_test_multiple_extends_with_multiple_extenders_and_single_target;

mod t010_test_multiple_extends_with_multiple_extenders_and_single_target;

mod t011_test_chained_extends;

mod t012_test_dynamic_extendee;

mod t013_test_dynamic_extendee;

mod t014_test_nested_target;

mod t015_test_target_with_child;

mod t016_test_class_unification;

mod t017_test_class_unification;

mod t018_test_id_unification;

mod t019_test_id_unification;

mod t020_test_universal_unification_with_simple_target;

mod t021_test_universal_unification_with_simple_target;

mod t022_test_universal_unification_with_simple_target;

mod t023_test_universal_unification_with_simple_target;

mod t024_test_universal_unification_with_simple_target;

mod t025_test_universal_unification_with_namespaceless_universal_target;

mod t026_test_universal_unification_with_namespaceless_universal_target;

mod t027_test_universal_unification_with_namespaceless_universal_target;

mod t028_test_universal_unification_with_namespaceless_universal_target;

mod t029_test_universal_unification_with_namespaceless_universal_target;

mod t030_test_universal_unification_with_namespaceless_universal_target;

mod t031_test_universal_unification_with_namespaced_universal_target;

mod t032_test_universal_unification_with_namespaced_universal_target;

mod t033_test_universal_unification_with_namespaced_universal_target;

mod t034_test_universal_unification_with_namespaceless_element_target;

mod t035_test_universal_unification_with_namespaceless_element_target;

mod t036_test_universal_unification_with_namespaceless_element_target;

mod t037_test_universal_unification_with_namespaceless_element_target;

mod t038_test_universal_unification_with_namespaceless_element_target;

mod t039_test_universal_unification_with_namespaceless_element_target;

mod t040_test_universal_unification_with_namespaced_element_target;

mod t041_test_universal_unification_with_namespaced_element_target;

mod t042_test_universal_unification_with_namespaced_element_target;

mod t043_test_element_unification_with_simple_target;

mod t044_test_element_unification_with_simple_target;

mod t045_test_element_unification_with_simple_target;

mod t046_test_element_unification_with_simple_target;

mod t047_test_element_unification_with_namespaceless_universal_target;

mod t048_test_element_unification_with_namespaceless_universal_target;

mod t049_test_element_unification_with_namespaceless_universal_target;

mod t050_test_element_unification_with_namespaceless_universal_target;

mod t051_test_element_unification_with_namespaceless_universal_target;

mod t052_test_element_unification_with_namespaceless_universal_target;

mod t053_test_element_unification_with_namespaced_universal_target;

mod t054_test_element_unification_with_namespaced_universal_target;

mod t055_test_element_unification_with_namespaced_universal_target;

mod t056_test_element_unification_with_namespaceless_element_target;

mod t057_test_element_unification_with_namespaceless_element_target;

mod t058_test_element_unification_with_namespaceless_element_target;

mod t059_test_element_unification_with_namespaceless_element_target;

mod t060_test_element_unification_with_namespaceless_element_target;

mod t061_test_element_unification_with_namespaceless_element_target;

mod t062_test_element_unification_with_namespaced_element_target;

mod t063_test_element_unification_with_namespaced_element_target;

mod t064_test_element_unification_with_namespaced_element_target;

mod t065_test_attribute_unification;

mod t066_test_attribute_unification;

mod t067_test_attribute_unification;

mod t068_test_attribute_unification;

mod t069_test_attribute_unification;

mod t070_test_pseudo_unification;

mod t071_test_pseudo_unification;

mod t072_test_pseudo_unification;

mod t073_test_pseudo_unification;

mod t074_test_pseudo_unification;

mod t075_test_pseudo_unification;

mod t076_test_pseudo_unification;

mod t077_test_pseudo_unification;

mod t078_test_pseudoelement_remains_at_end_of_selector;

mod t079_test_pseudoelement_remains_at_end_of_selector;

mod t080_test_pseudoclass_remains_at_end_of_selector;

mod t081_test_pseudoclass_remains_at_end_of_selector;

mod t082_test_not_remains_at_end_of_selector;

mod t083_test_pseudoelement_goes_lefter_than_pseudoclass;

mod t084_test_pseudoelement_goes_lefter_than_pseudoclass;

mod t085_test_pseudoelement_goes_lefter_than_not;

mod t086_1_test_pseudoelement_goes_lefter_than_not;

mod t086_test_pseudoelement_goes_lefter_than_not;

mod t087_test_negation_unification;

mod t088_test_negation_unification;

mod t089_test_negation_unification;

mod t090_test_comma_extendee;

mod t091_test_redundant_selector_elimination;

mod t094_test_long_extendee_runs_unification;

mod t095_test_long_extender;

mod t096_test_long_extender_runs_unification;

mod t097_test_nested_extender;

mod t098_test_nested_extender_runs_unification;

mod t099_test_nested_extender_alternates_parents;

mod t100_test_nested_extender_unifies_identical_parents;

mod t101_test_nested_extender_unifies_common_substring;

mod t102_test_nested_extender_unifies_common_subseq;

mod t103_test_nested_extender_chooses_first_subseq;

mod t104_test_nested_extender_counts_extended_subselectors;

mod t105_test_nested_extender_counts_extended_superselectors;

mod t106_test_nested_extender_with_child_selector;

mod t107_test_nested_extender_finds_common_selectors_around_child_selector;

mod t108_test_nested_extender_finds_common_selectors_around_child_selector;

mod t109_test_nested_extender_finds_common_selectors_around_adjacent_sibling;

mod t110_test_nested_extender_finds_common_selectors_around_adjacent_sibling;

mod t111_test_nested_extender_finds_common_selectors_around_adjacent_sibling;

mod t112_test_nested_extender_finds_common_selectors_around_sibling_selector;

mod t113_test_nested_extender_finds_common_selectors_around_sibling_selector;

mod t114_test_nested_extender_finds_common_selectors_around_sibling_selector;

mod t118_test_nested_extender_with_early_child_selectors_doesnt_subseq_them;

mod t119_test_nested_extender_with_early_child_selectors_doesnt_subseq_them;

mod t120_test_nested_extender_with_child_selector_unifies;

mod t121_test_nested_extender_with_child_selector_unifies;

mod t122_test_nested_extender_with_child_selector_unifies;

mod t123_test_nested_extender_with_early_child_selector;

mod t124_test_nested_extender_with_early_child_selector;

mod t125_test_nested_extender_with_early_child_selector;

mod t126_test_nested_extender_with_early_child_selector;

mod t127_test_nested_extender_with_early_child_selector;

mod t128_test_nested_extender_with_sibling_selector;

mod t129_test_nested_extender_with_hacky_selector;

mod t130_test_nested_extender_with_hacky_selector;

mod t131_test_nested_extender_merges_with_same_selector;

mod t132_test_nested_extender_with_child_selector_merges_with_same_selector;

mod t133_test_combinator_unification_for_hacky_combinators;

mod t134_test_combinator_unification_for_hacky_combinators;

mod t135_test_combinator_unification_for_hacky_combinators;

mod t136_test_combinator_unification_for_hacky_combinators;

mod t137_test_combinator_unification_for_hacky_combinators;

mod t138_test_combinator_unification_for_hacky_combinators;

mod t139_test_combinator_unification_for_hacky_combinators;

mod t140_test_combinator_unification_double_tilde;

mod t141_test_combinator_unification_double_tilde;

mod t142_test_combinator_unification_double_tilde;

mod t143_test_combinator_unification_double_tilde;

mod t144_test_combinator_unification_tilde_plus;

mod t145_test_combinator_unification_tilde_plus;

mod t146_test_combinator_unification_tilde_plus;

mod t147_test_combinator_unification_tilde_plus;

mod t148_test_combinator_unification_tilde_plus;

mod t149_test_combinator_unification_tilde_plus;

mod t150_test_combinator_unification_tilde_plus;

mod t151_test_combinator_unification_tilde_plus;

mod t152_test_combinator_unification_angle_sibling;

mod t153_test_combinator_unification_angle_sibling;

mod t154_test_combinator_unification_angle_sibling;

mod t155_test_combinator_unification_angle_sibling;

mod t156_test_combinator_unification_double_angle;

mod t157_test_combinator_unification_double_angle;

mod t158_test_combinator_unification_double_angle;

mod t159_test_combinator_unification_double_angle;

mod t160_test_combinator_unification_double_plus;

mod t161_test_combinator_unification_double_plus;

mod t162_test_combinator_unification_double_plus;

mod t163_test_combinator_unification_double_plus;

mod t164_test_combinator_unification_angle_space;

mod t165_test_combinator_unification_angle_space;

mod t166_test_combinator_unification_angle_space;

mod t167_test_combinator_unification_angle_space;

mod t168_test_combinator_unification_angle_space;

mod t169_test_combinator_unification_angle_space;

mod t170_test_combinator_unification_plus_space;

mod t171_test_combinator_unification_plus_space;

mod t172_test_combinator_unification_plus_space;

mod t173_test_combinator_unification_plus_space;

mod t174_test_combinator_unification_plus_space;

mod t175_test_combinator_unification_plus_space;

mod t176_test_combinator_unification_nested;

mod t177_test_combinator_unification_nested;

mod t178_test_combinator_unification_with_newlines;

mod t179_test_extend_self_loop;

mod t180_test_basic_extend_loop;

mod t181_test_three_level_extend_loop;

mod t182_test_nested_extend_loop;

mod t183_test_multiple_extender_merges_with_superset_selector;

mod t184_test_control_flow_if;

mod t185_test_control_flow_for;

mod t186_test_control_flow_while;

mod t187_test_basic_placeholder_selector;

mod t188_test_unused_placeholder_selector;

mod t189_test_placeholder_descendant_selector;

mod t190_test_semi_placeholder_selector;

mod t191_test_placeholder_selector_with_multiple_extenders;

mod t192_test_placeholder_interpolation;

mod t193_test_media_in_placeholder_selector;

mod t194_test_extend_within_media;

mod t195_test_extend_within_unknown_directive;

mod t196_test_extend_within_nested_directives;

mod t197_test_extend_within_disparate_media;

mod t198_test_extend_within_disparate_unknown_directive;

mod t199_test_extend_within_disparate_nested_directives;

mod t206_test_extend_succeeds_when_one_extension_fails_but_others_dont;

mod t207_test_optional_extend_succeeds_when_extendee_doesnt_exist;

mod t208_test_optional_extend_succeeds_when_extension_fails;

mod t209_test_pseudo_element_superselector;

mod t210_test_pseudo_element_superselector;

mod t211_test_pseudo_element_superselector;

mod t212_test_pseudo_element_superselector;

mod t213_test_pseudo_element_superselector;

mod t214_test_pseudo_element_superselector;

mod t215_test_multiple_source_redundancy_elimination;

mod t216_test_nested_sibling_extend;

mod t217_test_parent_and_sibling_extend;

mod t218_test_nested_extend_specificity;

mod t219_test_nested_double_extend_optimization;

mod t220_test_extend_in_double_nested_media_query;

mod t221_test_partially_failed_extend;

mod t222_test_newline_near_combinator;

mod t223_test_duplicated_selector_with_newlines;

mod t224_test_nested_selector_with_child_selector_hack_extendee;

mod t225_test_nested_selector_with_child_selector_hack_extender;

mod t226_test_nested_selector_with_child_selector_hack_extender_and_extendee;

mod t227_test_nested_with_child_hack_extender_and_sibling_extendee;

mod t228_test_nested_with_child_selector_hack_extender_extendee_newline;

mod t229_test_extended_parent_and_child_redundancy_elimination;

mod t230_test_extend_redundancy_elimination_when_it_would_reduce_specificity;

mod t231_test_extend_redundancy_elimination_when_it_would_preserve_specificity;

mod t232_test_extend_redundancy_elimination_never_eliminates_base_selector;

mod t233_test_extend_cross_branch_redundancy_elimination;

mod t234_test_extend_cross_branch_redundancy_elimination;

mod t235_extend_with_universal_selector;

mod t236_extend_with_universal_selector_empty_namespace;

mod t237_extend_with_universal_selector_different_namespace;

mod t238_unify_root_pseudoelement;

mod compound_unification_in_not;

mod does_not_move_page_block_in_media;

mod escaped_selector;

mod extend_extender;

mod extend_loop;

mod extend_result_of_extend;

mod extend_self;

mod fake_pseudo_element_order;

mod issue_146;

mod nested_compound_unification;

mod not_into_not_not;

mod selector_list;
