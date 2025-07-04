// Copyright 2025 Google LLC
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

#[derive(Debug)]
struct ArenaHolder<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T: PartialEq + ?Sized> PartialEq<T> for ArenaHolder<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.value == other
    }
}

struct Strukt {
    a_field: String,
}

impl<'a> ArenaHolder<'a, Strukt> {
    fn get_a_field(&self) -> ArenaHolder<'_, str> {
        ArenaHolder { value: &self.value.a_field }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test() {
        let arena = [Strukt { a_field: "hello".into() }];

        let holder = ArenaHolder { value: &arena[0] };

        let inner_holder = holder.get_a_field();

        expect_that!(inner_holder, eq("hello"));
    }
}
