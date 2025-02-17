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
mod core {}

#[cfg(test)]
mod tests {
    // NOTE: this test module intentionally does not import items from
    // `googletest` to ensure that the macros do not depend on other items from the
    // library being imported.
    use googletest::gtest;

    #[gtest]
    fn verify_that_works() -> googletest::Result<()> {
        googletest::verify_that!(2 + 2, googletest::matchers::eq(4))
    }

    #[gtest]
    fn verify_pred_works() -> googletest::Result<()> {
        fn pred() -> bool {
            true
        }
        googletest::verify_pred!(pred())
    }

    #[gtest]
    fn fail_works() -> googletest::Result<()> {
        googletest::fail!()
    }

    #[gtest]
    fn succeed_works() {
        googletest::succeed!();
    }

    #[gtest]
    fn add_failure_works() {
        googletest::add_failure!("message");
    }

    #[gtest]
    fn add_failure_at_works() {
        googletest::add_failure_at!("hello.rs", 12, 34, "message");
    }

    #[gtest]
    fn verify_true_works() -> googletest::Result<()> {
        googletest::verify_true!(true)
    }

    #[gtest]
    fn expect_true_works() {
        googletest::expect_true!(true);
    }

    #[gtest]
    fn verify_false_works() -> googletest::Result<()> {
        googletest::verify_false!(false)
    }

    #[gtest]
    fn expect_false_works() {
        googletest::expect_false!(false);
    }

    #[gtest]
    fn verify_eq_works() -> googletest::Result<()> {
        googletest::verify_eq!(2 + 2, 4)
    }

    #[gtest]
    fn expect_eq_works() {
        googletest::expect_eq!(2 + 2, 4);
    }

    #[gtest]
    fn verify_ne_works() -> googletest::Result<()> {
        googletest::verify_ne!(2 + 2, 5)
    }

    #[gtest]
    fn expect_ne_works() {
        googletest::expect_ne!(2 + 2, 5);
    }

    #[gtest]
    fn verify_lt_works() -> googletest::Result<()> {
        googletest::verify_lt!(2 + 2, 5)
    }

    #[gtest]
    fn expect_lt_works() {
        googletest::expect_lt!(2 + 2, 5);
    }

    #[gtest]
    fn verify_le_works() -> googletest::Result<()> {
        googletest::verify_le!(2 + 2, 4)
    }

    #[gtest]
    fn expect_le_works() {
        googletest::expect_le!(2 + 2, 4);
    }

    #[gtest]
    fn verify_gt_works() -> googletest::Result<()> {
        googletest::verify_gt!(2 + 2, 3)
    }

    #[gtest]
    fn expect_gt_works() {
        googletest::expect_gt!(2 + 2, 3);
    }

    #[gtest]
    fn verify_ge_works() -> googletest::Result<()> {
        googletest::verify_ge!(2 + 2, 4)
    }

    #[gtest]
    fn expect_ge_works() {
        googletest::expect_ge!(2 + 2, 4);
    }

    #[gtest]
    fn verify_float_eq_works() -> googletest::Result<()> {
        googletest::verify_float_eq!(1.0, 1.0)
    }

    #[gtest]
    fn expect_float_eq_works() {
        googletest::expect_float_eq!(1.0, 1.0);
    }

    #[gtest]
    fn verify_near_works() -> googletest::Result<()> {
        googletest::verify_near!(42.0, 42.001, 0.1)
    }

    #[gtest]
    fn expect_near_works() {
        googletest::expect_near!(42.0, 42.001, 0.1);
    }

    #[gtest]
    fn assert_that_works() {
        googletest::assert_that!(2 + 2, googletest::matchers::eq(4));
    }

    #[gtest]
    fn assert_pred_works() {
        fn pred() -> bool {
            true
        }
        googletest::assert_pred!(pred());
    }

    #[gtest]
    fn expect_that_works() {
        googletest::expect_that!(2 + 2, googletest::matchers::eq(4));
    }

    #[gtest]
    fn expect_pred_works() {
        fn pred() -> bool {
            true
        }
        googletest::expect_pred!(pred());
    }
}
