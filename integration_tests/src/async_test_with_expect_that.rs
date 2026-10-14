// Copyright 2023 Google LLC
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

#[deny(warnings)]
#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use std::time::Duration;
    use tokio::time::sleep;

    async fn brief_sleep() {
        sleep(Duration::from_millis(1)).await;
    }

    #[gtest]
    #[tokio::test]
    async fn async_test_with_no_output() {
        brief_sleep().await;
    }

    #[gtest]
    #[tokio::test]
    async fn async_test_failure_with_non_fatal_assertion() -> Result<()> {
        brief_sleep().await;
        expect_that!(2, eq(3));
        Ok(())
    }

    #[gtest]
    #[tokio::test]
    async fn async_test_failure_with_fatal_assertion() -> Result<()> {
        brief_sleep().await;
        verify_that!(3, eq(4))?;
        Ok(())
    }

    struct VerifyFiveOnTearDown(i32);

    impl Fixture for VerifyFiveOnTearDown {
        fn set_up() -> Result<Self> {
            Ok(VerifyFiveOnTearDown(0))
        }

        fn tear_down(self) -> Result<()> {
            verify_eq!(self.0, 5)?;
            Ok(())
        }
    }

    #[gtest]
    #[tokio::test]
    async fn async_test_with_ok_fixture(f: &mut VerifyFiveOnTearDown) -> Result<()> {
        brief_sleep().await;
        f.0 = 5;
        Ok(())
    }

    #[gtest]
    #[tokio::test]
    async fn async_test_with_err_fixture(f: &mut VerifyFiveOnTearDown) -> Result<()> {
        brief_sleep().await;
        f.0 = 6;
        Ok(())
    }

    #[gtest]
    #[tokio::test]
    async fn async_test_with_scoped_trace() -> Result<()> {
        scoped_trace!("Outer trace");
        brief_sleep().await;
        {
            scoped_trace!("Inner trace");
            brief_sleep().await;
            let traces = googletest::internal::scoped_trace::get_scoped_traces();
            verify_eq!(traces.len(), 2)?;
            verify_eq!(traces[0].message, "Outer trace")?;
            verify_eq!(traces[1].message, "Inner trace")?;
        }
        brief_sleep().await;
        let traces = googletest::internal::scoped_trace::get_scoped_traces();
        verify_eq!(traces.len(), 1)?;
        verify_eq!(traces[0].message, "Outer trace")?;
        Ok(())
    }

    fn sync_subroutine_with_trace() {
        scoped_trace!("Sync subroutine trace");
        let traces = googletest::internal::scoped_trace::get_scoped_traces();
        assert!(traces.iter().any(|t| t.message == "Sync subroutine trace"));
    }

    #[gtest]
    #[tokio::test]
    async fn async_test_mixed_scoped_trace() -> Result<()> {
        scoped_trace!("Outer async trace");
        brief_sleep().await;

        sync_subroutine_with_trace();

        {
            scoped_trace!("Inner async trace");
            brief_sleep().await;
            let traces = googletest::internal::scoped_trace::get_scoped_traces();
            verify_eq!(traces.len(), 2)?;
            verify_eq!(traces[0].message, "Outer async trace")?;
            verify_eq!(traces[1].message, "Inner async trace")?;
        }

        brief_sleep().await;
        let traces = googletest::internal::scoped_trace::get_scoped_traces();
        verify_eq!(traces.len(), 1)?;
        verify_eq!(traces[0].message, "Outer async trace")?;
        Ok(())
    }
}
