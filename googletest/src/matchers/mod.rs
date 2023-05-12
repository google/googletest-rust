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

pub mod all_matcher;
pub mod anything_matcher;
pub mod conjunction_matcher;
pub mod container_eq_matcher;
pub mod contains_matcher;
pub mod contains_regex_matcher;
pub mod disjunction_matcher;
pub mod display_matcher;
pub mod each_matcher;
pub mod elements_are_matcher;
pub mod empty_matcher;
pub mod eq_deref_of_matcher;
pub mod eq_matcher;
pub mod err_matcher;
pub mod field_matcher;
pub mod ge_matcher;
pub mod gt_matcher;
pub mod has_entry_matcher;
pub mod is_nan_matcher;
pub mod le_matcher;
pub mod len_matcher;
pub mod lt_matcher;
pub mod matches_pattern;
pub mod matches_regex_matcher;
pub mod near_matcher;
pub mod none_matcher;
pub mod not_matcher;
pub mod ok_matcher;
pub mod points_to_matcher;
pub mod pointwise_matcher;
pub mod predicate_matcher;
pub mod property_matcher;
pub mod some_matcher;
pub mod str_matcher;
pub mod subset_of_matcher;
pub mod superset_of_matcher;
pub mod tuple_matcher;
pub mod unordered_elements_are_matcher;

pub use anything_matcher::anything;
pub use container_eq_matcher::container_eq;
pub use contains_matcher::contains;
pub use contains_regex_matcher::contains_regex;
pub use display_matcher::displays_as;
pub use each_matcher::each;
pub use empty_matcher::empty;
pub use eq_deref_of_matcher::eq_deref_of;
pub use eq_matcher::eq;
pub use err_matcher::err;
pub use ge_matcher::ge;
pub use gt_matcher::gt;
pub use has_entry_matcher::has_entry;
pub use is_nan_matcher::is_nan;
pub use le_matcher::le;
pub use len_matcher::len;
pub use lt_matcher::lt;
pub use matches_regex_matcher::matches_regex;
pub use near_matcher::{approx_eq, near};
pub use none_matcher::none;
pub use not_matcher::not;
pub use ok_matcher::ok;
pub use points_to_matcher::points_to;
pub use predicate_matcher::{predicate, PredicateMatcher};
pub use some_matcher::some;
pub use str_matcher::{contains_substring, ends_with, starts_with, StrMatcherConfigurator};
pub use subset_of_matcher::subset_of;
pub use superset_of_matcher::superset_of;
