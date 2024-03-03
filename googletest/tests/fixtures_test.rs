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

use googletest::prelude::*;

TEST!(MyTestSuite, MyTest, { verify_that!(1, eq(1)) });

struct MyFixture;

impl TestFixture for MyFixture {
    fn create() -> Self {
        MyFixture
    }
}

TEST_F!(MyFixture, FixturedTest, { verify_that!(1, eq(1)) });

struct MyOtherFixture;

impl TestFixture for MyOtherFixture {
    fn create() -> Self {
        Self
    }
}

TEST_F!(MyOtherFixture, FixturedTest, { verify_that!(1, eq(1)) });

struct ParamFixture<T>(std::marker::PhantomData<T>);

impl<T> TestFixture for ParamFixture<T> {
    fn create() -> Self {
        Self(Default::default())
    }
}
TEST_F!(ParamFixture<i32>, FixturedTest, { verify_that!(1, eq(1)) });
TEST_F!(ParamFixture<(i32, i32)>, FixturedTest, { verify_that!(1, eq(1)) });

struct ConstParamFixture<const N: i32>;
impl<const N: i32> TestFixture for ConstParamFixture<N> {
    fn create() -> Self {
        Self
    }
}
TEST_F!(ConstParamFixture<123>, FixturedTest, { verify_that!(1, eq(1)) });

struct FakeFixture {
    set_up_has_run: bool,
    test_case_has_run: bool,
}

impl TestFixture for FakeFixture {
    fn create() -> Self {
        Self { set_up_has_run: false, test_case_has_run: false }
    }
    fn set_up(&mut self) {
        self.set_up_has_run = true;
    }

    fn tear_down(&mut self) -> googletest::Result<()> {
        verify_that!(self.test_case_has_run, eq(true))
    }
}

TEST_F!(FakeFixture, BeforeAndAfterAreCalled, {
    verify_that!(self.set_up_has_run, eq(true))?;
    self.test_case_has_run = true;
    Ok(())
});
