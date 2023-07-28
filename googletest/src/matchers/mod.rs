// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! All built-in matchers of this crate are in submodules of this module.

pub mod all_matcher;
pub mod any_matcher;
mod anything_matcher;
mod char_count_matcher;
pub mod conjunction_matcher;
mod container_eq_matcher;
mod contains_matcher;
mod contains_regex_matcher;
pub mod disjunction_matcher;
mod display_matcher;
mod each_matcher;
pub mod elements_are_matcher;
mod empty_matcher;
mod eq_deref_of_matcher;
mod eq_matcher;
mod err_matcher;
pub mod field_matcher;
mod ge_matcher;
mod gt_matcher;
mod has_entry_matcher;
pub mod is_matcher;
mod is_nan_matcher;
mod le_matcher;
mod len_matcher;
mod lt_matcher;
mod matches_pattern;
mod matches_regex_matcher;
mod near_matcher;
mod none_matcher;
mod not_matcher;
mod ok_matcher;
mod points_to_matcher;
pub mod pointwise_matcher;
mod predicate_matcher;
pub mod property_matcher;
mod some_matcher;
mod str_matcher;
mod subset_of_matcher;
mod superset_of_matcher;
mod tuple_matcher;
pub mod unordered_elements_are_matcher;

pub use anything_matcher::anything;
pub use char_count_matcher::char_count;
pub use container_eq_matcher::container_eq;
pub use contains_matcher::{contains, ContainsMatcher};
pub use contains_regex_matcher::contains_regex;
pub use display_matcher::displays_as;
pub use each_matcher::each;
pub use empty_matcher::empty;
pub use eq_deref_of_matcher::eq_deref_of;
pub use eq_matcher::{eq, EqMatcher};
pub use err_matcher::err;
pub use ge_matcher::ge;
pub use gt_matcher::gt;
pub use has_entry_matcher::has_entry;
pub use is_nan_matcher::is_nan;
pub use le_matcher::le;
pub use len_matcher::len;
pub use lt_matcher::lt;
pub use matches_regex_matcher::matches_regex;
pub use near_matcher::{approx_eq, near, NearMatcher};
pub use none_matcher::none;
pub use not_matcher::not;
pub use ok_matcher::ok;
pub use points_to_matcher::points_to;
pub use predicate_matcher::{predicate, PredicateMatcher};
pub use some_matcher::some;
pub use str_matcher::{
    contains_substring, ends_with, starts_with, StrMatcher, StrMatcherConfigurator,
};
pub use subset_of_matcher::subset_of;
pub use superset_of_matcher::superset_of;
