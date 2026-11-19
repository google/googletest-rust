// Copyright 2026 Google LLC
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

use googletest::fixtures::tear_down_static_fixtures;
use googletest::prelude::*;
use std::sync::Mutex;

/// The set-ups and tear-downs of the fixtures below, in the order they ran.
static EVENTS: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

fn record(event: &'static str) {
    EVENTS.lock().unwrap().push(event);
}

struct First;

impl StaticFixture for First {
    fn set_up_once() -> googletest::Result<Self> {
        record("set up First");
        Ok(Self)
    }

    fn tear_down_once(&self) {
        record("tear down First");
    }
}

struct Second;

impl StaticFixture for Second {
    fn set_up_once() -> googletest::Result<Self> {
        record("set up Second");
        Ok(Self)
    }

    fn tear_down_once(&self) {
        record("tear down Second");
    }
}

struct FailingSetUp;

impl StaticFixture for FailingSetUp {
    fn set_up_once() -> googletest::Result<Self> {
        record("set up FailingSetUp");
        Err::<Self, _>("set_up_once failed").or_fail()
    }

    fn tear_down_once(&self) {
        record("tear down FailingSetUp");
    }
}

struct WithoutTearDown;

impl StaticFixture for WithoutTearDown {
    fn set_up_once() -> googletest::Result<Self> {
        record("set up WithoutTearDown");
        Ok(Self)
    }
}

// All the checks are in a single test case, because `tear_down_static_fixtures`
// applies to the whole process while the test cases of a binary run
// concurrently. A `tear_down_once` which logs a failure is covered by the tests
// of //devtools/rust/testrunner, since the default Rust test harness has no
// test context outside of a test case.
#[test]
fn tear_down_static_fixtures_tears_down_each_set_up_fixture_once() -> googletest::Result<()> {
    // Without fixtures, there is nothing to tear down.
    tear_down_static_fixtures();
    verify_that!(*EVENTS.lock().unwrap(), is_empty())?;

    // Each fixture is set up once, however often it is used.
    <&'static First as Fixture>::set_up()?;
    <&'static First as Fixture>::set_up()?;
    <&'static Second as Fixture>::set_up()?;
    verify_that!(<&'static FailingSetUp as Fixture>::set_up().is_err(), eq(true))?;
    <&'static WithoutTearDown as Fixture>::set_up()?;

    // The fixtures are torn down in the reverse order of their set-up. A
    // fixture whose set-up failed is not torn down.
    tear_down_static_fixtures();
    verify_that!(
        *EVENTS.lock().unwrap(),
        elements_are![
            eq(&"set up First"),
            eq(&"set up Second"),
            eq(&"set up FailingSetUp"),
            eq(&"set up WithoutTearDown"),
            eq(&"tear down Second"),
            eq(&"tear down First"),
        ]
    )?;

    // Fixtures are torn down at most once.
    EVENTS.lock().unwrap().clear();
    tear_down_static_fixtures();
    verify_that!(*EVENTS.lock().unwrap(), is_empty())?;

    // A fixture used after the tear-down is set up again.
    <&'static First as Fixture>::set_up()?;
    verify_that!(*EVENTS.lock().unwrap(), elements_are![eq(&"set up First")])?;
    tear_down_static_fixtures();
    verify_that!(
        *EVENTS.lock().unwrap(),
        elements_are![eq(&"set up First"), eq(&"tear down First")]
    )
}
