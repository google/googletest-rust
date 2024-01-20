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

/// Counts the number of elements in `value`.
///
/// This uses [`Iterator::size_hint`] when that function returns an
/// unambiguous answer, i.e., the upper bound exists and the lower and upper
/// bounds agree. Otherwise it iterates through `value` and counts the
/// elements.
pub(crate) fn count_elements<'b, ContainerT: ?Sized>(value: &'b ContainerT) -> usize
where
    &'b ContainerT: IntoIterator,
{
    let iterator = value.into_iter();
    if let (lower, Some(higher)) = iterator.size_hint() {
        if lower == higher {
            return lower;
        }
    }
    iterator.count()
}
