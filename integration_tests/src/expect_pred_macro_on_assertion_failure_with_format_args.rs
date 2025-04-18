// Copyright 2024 Google LLC
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

fn main() {}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    #[gtest]
    fn should_fail() {
        let a = 1;
        let b = 2;
        let extra_information = "extra information";

        expect_pred!(eq_predicate(a, b), "assertion failed: {extra_information}")
    }

    fn eq_predicate(a: i32, b: i32) -> bool {
        a == b
    }
}
