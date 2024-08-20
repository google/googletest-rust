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

// Shadow important modules that the macros use.
mod std {}
mod ctor {}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    #[gtest]
    fn verify_that_works() -> Result<()> {
        verify_that!(2 + 2, eq(4))
    }

    #[gtest]
    fn verify_pred_works() -> Result<()> {
        fn pred() -> bool {
            true
        }
        verify_pred!(pred())
    }

    #[gtest]
    fn fail_works() -> Result<()> {
        fail!()
    }

    #[gtest]
    fn succeed_works() {
        succeed!();
    }

    #[gtest]
    fn add_failure_works() {
        add_failure!("message");
    }

    #[gtest]
    fn add_failure_at_works() {
        add_failure_at!("hello.rs", 12, 34, "message");
    }

    #[gtest]
    fn verify_true_works() -> Result<()> {
        verify_true!(true)
    }

    #[gtest]
    fn expect_true_works() {
        expect_true!(true);
    }

    #[gtest]
    fn verify_false_works() -> Result<()> {
        verify_false!(false)
    }

    #[gtest]
    fn expect_false_works() {
        expect_false!(false);
    }

    #[gtest]
    fn verify_eq_works() -> Result<()> {
        verify_eq!(2 + 2, 4)
    }

    #[gtest]
    fn expect_eq_works() {
        expect_eq!(2 + 2, 4);
    }

    #[gtest]
    fn verify_ne_works() -> Result<()> {
        verify_ne!(2 + 2, 5)
    }

    #[gtest]
    fn expect_ne_works() {
        expect_ne!(2 + 2, 5);
    }

    #[gtest]
    fn verify_lt_works() -> Result<()> {
        verify_lt!(2 + 2, 5)
    }

    #[gtest]
    fn expect_lt_works() {
        expect_lt!(2 + 2, 5);
    }

    #[gtest]
    fn verify_le_works() -> Result<()> {
        verify_le!(2 + 2, 4)
    }

    #[gtest]
    fn expect_le_works() {
        expect_le!(2 + 2, 4);
    }

    #[gtest]
    fn verify_gt_works() -> Result<()> {
        verify_gt!(2 + 2, 3)
    }

    #[gtest]
    fn expect_gt_works() {
        expect_gt!(2 + 2, 3);
    }

    #[gtest]
    fn verify_ge_works() -> Result<()> {
        verify_ge!(2 + 2, 4)
    }

    #[gtest]
    fn expect_ge_works() {
        expect_ge!(2 + 2, 4);
    }

    #[gtest]
    fn verify_float_eq_works() -> Result<()> {
        verify_float_eq!(1.0, 1.0)
    }

    #[gtest]
    fn expect_float_eq_works() {
        expect_float_eq!(1.0, 1.0);
    }

    #[gtest]
    fn verify_near_works() -> Result<()> {
        verify_near!(42.0, 42.001, 0.1)
    }

    #[gtest]
    fn expect_near_works() {
        expect_near!(42.0, 42.001, 0.1);
    }

    #[gtest]
    fn assert_that_works() {
        assert_that!(2 + 2, eq(4));
    }

    #[gtest]
    fn assert_pred_works() {
        fn pred() -> bool {
            true
        }
        assert_pred!(pred());
    }

    #[gtest]
    fn expect_that_works() {
        expect_that!(2 + 2, eq(4));
    }

    #[gtest]
    fn expect_pred_works() {
        fn pred() -> bool {
            true
        }
        expect_pred!(pred());
    }
}
