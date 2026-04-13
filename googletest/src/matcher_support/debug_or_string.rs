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

//! This module provides a way to format a value as a debug string, or as a
// regular string if the Debug trait is not implemented.
//
// This is useful for matchers that want to format the actual value in a
// user-friendly way, but don't want to require the Debug trait for all types.

#[macro_export]
macro_rules! debug_or_string {
    ($actual:expr, $fallback:expr) => {{
        #[allow(unused_imports)]
        use $crate::matcher_support::debug_or_string::format_helper::DebugFallback;
        $crate::matcher_support::debug_or_string::format_helper::DebugWrapper(&$actual)
            .debug_or_string($fallback)
    }};
}

pub(crate) mod format_helper {
    use std::fmt::Debug;

    // Create a wrapper struct to hold the value.
    #[allow(dead_code)]
    pub struct DebugWrapper<'a, T>(pub &'a T);

    // High Priority: Define an INHERENT method for Debug types.
    // The compiler will prefer this if T implements Debug.
    impl<'a, T: Debug> DebugWrapper<'a, T> {
        #[allow(dead_code)]
        pub fn debug_or_string(&self, _fallback: &str) -> String {
            crate::matcher::format_actual(self.0)
        }
    }

    // Low Priority: Define a TRAIT for the fallback case.
    pub trait DebugFallback {
        fn debug_or_string(&self, fallback: &str) -> String;
    }

    // This matches any type T, but inherent methods always win if available.
    impl<'a, T> DebugFallback for DebugWrapper<'a, T> {
        fn debug_or_string(&self, fallback: &str) -> String {
            fallback.to_string()
        }
    }
}
