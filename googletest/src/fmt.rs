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

/// Functions for use only by the procedural macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use std::fmt::{Debug, Write};

    /// Wrapper to allow for inherent-method specialization based on whether a
    /// type implements `Debug`.
    pub struct FormatWrapper<'a, T: ?Sized>(pub &'a T);

    /// Default implementation to render values that implement `Debug`.
    ///
    /// Used for autoref specialization to conditionally
    /// render only values that implement `Debug`. See also
    /// [`FormatNonDebugFallback`].
    impl<'a, T: Debug + ?Sized> FormatWrapper<'a, T> {
        #[track_caller]
        pub fn __googletest_write_expr_value(&self, output: &mut dyn Write, expr_label: &str) {
            write!(output, "\n  {} = {:?},", expr_label, self.0)
                .expect("Formatting to String should never fail");
        }
    }

    /// Fallback implementation for rendering values for non-`Debug` types..
    ///
    /// Used for inherent-method specialization to conditionally render only
    /// values that implement `Debug`. See also the specialized inherent impl on
    /// [`FormatWrapper`] above.
    pub trait FormatNonDebugFallback {
        fn __googletest_write_expr_value(&self, output: &mut dyn Write, expr_label: &str);
    }

    impl<'a, T: ?Sized> FormatNonDebugFallback for FormatWrapper<'a, T> {
        #[track_caller]
        fn __googletest_write_expr_value(&self, output: &mut dyn Write, expr_label: &str) {
            write!(output, "\n  {} does not implement Debug,", expr_label)
                .expect("Formatting to String should never fail");
        }
    }

    #[macro_export]
    macro_rules! __googletest__write_expr_value(
        ($output:expr, $expr_str:expr, $value:expr $(,)?) => {
            {
                use $crate::fmt::internal::FormatNonDebugFallback as _;
                $crate::fmt::internal::FormatWrapper(&$value)
                    .__googletest_write_expr_value(&mut $output, $expr_str)
            }
        }
    );
    pub use __googletest__write_expr_value;
}
