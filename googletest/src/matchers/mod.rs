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

#[cfg(not(google3))]
pub mod all_matcher;
#[cfg(not(google3))]
pub mod anything_matcher;
#[cfg(not(google3))]
pub mod conjunction_matcher;
#[cfg(not(google3))]
pub mod container_eq_matcher;
#[cfg(not(google3))]
pub mod contains_matcher;
#[cfg(not(google3))]
pub mod contains_regex_matcher;
#[cfg(not(google3))]
pub mod description;
#[cfg(not(google3))]
pub mod disjunction_matcher;
#[cfg(not(google3))]
pub mod display_matcher;
#[cfg(not(google3))]
pub mod each_matcher;
#[cfg(not(google3))]
pub mod elements_are_matcher;
#[cfg(not(google3))]
pub mod empty_matcher;
#[cfg(not(google3))]
pub mod eq_matcher;
#[cfg(not(google3))]
pub mod err_matcher;
#[cfg(not(google3))]
pub mod field_matcher;
#[cfg(not(google3))]
pub mod ge_matcher;
#[cfg(not(google3))]
pub mod gt_matcher;
#[cfg(not(google3))]
pub mod has_entry_matcher;
#[cfg(not(google3))]
pub mod has_size;
#[cfg(not(google3))]
pub mod is_nan_matcher;
#[cfg(not(google3))]
pub mod le_matcher;
#[cfg(not(google3))]
pub mod lt_matcher;
#[cfg(not(google3))]
pub mod matches_pattern;
#[cfg(not(google3))]
pub mod matches_regex_matcher;
#[cfg(not(google3))]
pub mod near_matcher;
#[cfg(not(google3))]
pub mod none_matcher;
#[cfg(not(google3))]
pub mod not_matcher;
#[cfg(not(google3))]
pub mod ok_matcher;
#[cfg(not(google3))]
pub mod points_to_matcher;
#[cfg(not(google3))]
pub mod pointwise_matcher;
#[cfg(not(google3))]
pub mod predicate_matcher;
#[cfg(not(google3))]
pub mod size_matcher;
#[cfg(not(google3))]
pub mod some_matcher;
#[cfg(not(google3))]
pub mod str_matcher;
#[cfg(not(google3))]
pub mod subset_of_matcher;
#[cfg(not(google3))]
pub mod superset_of_matcher;
#[cfg(not(google3))]
pub mod tuple_matcher;
#[cfg(not(google3))]
pub mod unordered_elements_are_matcher;

#[cfg(google3)]
pub use all_matcher::all;
pub use anything_matcher::anything;
pub use conjunction_matcher::AndMatcherExt;
pub use container_eq_matcher::container_eq;
pub use contains_matcher::contains;
pub use contains_regex_matcher::contains_regex;
pub use disjunction_matcher::OrMatcherExt;
pub use display_matcher::displays_as;
pub use each_matcher::each;
#[cfg(google3)]
pub use elements_are_matcher::elements_are;
pub use empty_matcher::empty;
pub use eq_matcher::eq;
pub use err_matcher::err;
#[cfg(google3)]
pub use field_matcher::field;
pub use ge_matcher::ge;
pub use gt_matcher::gt;
pub use has_entry_matcher::has_entry;
pub use is_nan_matcher::is_nan;
pub use le_matcher::le;
pub use lt_matcher::lt;
#[cfg(google3)]
pub use matches_pattern::matches_pattern;
pub use matches_regex_matcher::matches_regex;
pub use near_matcher::{approx_eq, near};
pub use none_matcher::none;
pub use not_matcher::not;
pub use ok_matcher::ok;
pub use points_to_matcher::points_to;
#[cfg(google3)]
pub use pointwise_matcher::pointwise;
#[cfg(google3)]
pub use predicate_matcher::{predicate, PredicateMatcher};
pub use size_matcher::size;
pub use some_matcher::some;
pub use str_matcher::{contains_substring, ends_with, starts_with};
pub use subset_of_matcher::subset_of;
pub use superset_of_matcher::superset_of;
#[cfg(google3)]
pub use tuple_matcher::tuple;
#[cfg(google3)]
pub use unordered_elements_are_matcher::{contains_each, unordered_elements_are};
