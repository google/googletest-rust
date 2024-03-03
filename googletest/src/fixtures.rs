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

/// An interface for setting up and tearing down data used in multiple tests.
///
/// The test fixture passed to [`TEST_F`](googletest_macro::TEST_F) must
/// implement this interface.
pub trait TestFixture {
    /// Create the the test fixture
    fn create() -> Self;

    /// Set up the data in the test fixture
    fn set_up(&mut self) {}

    /// Tear down the data in the test fixture. Can fail the test by returning
    /// an error.
    fn tear_down(&mut self) -> crate::Result<()> {
        Ok(())
    }
}
