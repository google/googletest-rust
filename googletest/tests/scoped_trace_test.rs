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

#[cfg(test)]
mod tests {
    use googletest::internal::scoped_trace::get_scoped_traces;
    use googletest::prelude::*;

    #[gtest]
    fn test_scoped_trace_non_fatal() -> googletest::Result<()> {
        let result = {
            scoped_trace!("First trace");
            verify_eq!(1, 2)
        };

        verify_that!(result, err(displays_as(contains_substring("First trace"))))?;

        verify_that!(result, err(displays_as(contains_substring("Google Test trace:"))))
    }

    #[gtest]
    fn test_scoped_trace_nesting() {
        scoped_trace!("Outer trace");
        {
            scoped_trace!("Inner trace");
            let traces = get_scoped_traces();
            assert_eq!(traces.len(), 2);
            assert_eq!(traces[0].message, "Outer trace");
            assert_eq!(traces[1].message, "Inner trace");
        }
        let traces = get_scoped_traces();
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].message, "Outer trace");
    }
}
