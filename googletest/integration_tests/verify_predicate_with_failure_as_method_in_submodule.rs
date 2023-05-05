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

fn main() {}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    #[googletest::test]
    fn verify_predicate_on_method_in_submodule_with_failure() -> Result<()> {
        let a = 1;
        let b = 2;
        verify_pred!(a_submodule::A_STRUCT_IN_SUBMODULE.eq_predicate_as_method(a, b))
    }

    struct AStruct {}

    impl AStruct {
        fn eq_predicate_as_method(&self, a: i32, b: i32) -> bool {
            a == b
        }
    }

    mod a_submodule {
        pub(super) static A_STRUCT_IN_SUBMODULE: super::AStruct = super::AStruct {};
    }
}
