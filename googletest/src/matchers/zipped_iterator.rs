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

/// Zips up two iterators into a single iterator of pairs.
///
/// This is identical to [`Iterator::zip`] except that this version allows the
/// caller to determine whether the two iterators had mismatching sizes using
/// the method [`ZippedIterator::has_size_mismatch`].
///
/// [`Iterator::zip`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip
pub(crate) fn zip<I1, I2>(left: I1, right: I2) -> ZippedIterator<I1, I2> {
    ZippedIterator { left, right, has_size_mismatch: false, consumed_elements: 0 }
}

/// An iterator over pairs of the elements of two constituent iterators, which
/// keeps track of whether the two iterators have the same size.
///
/// This is identical to [`Zip`] except that it allows the caller to determine
/// whether the two iterators had mismatching sizes using the method
/// [`ZippedIterator::has_size_mismatch`].
///
/// [`Zip`]: https://doc.rust-lang.org/std/iter/struct.Zip.html
pub(crate) struct ZippedIterator<I1, I2> {
    left: I1,
    right: I2,
    has_size_mismatch: bool,
    consumed_elements: usize,
}

impl<I1: Iterator, I2> ZippedIterator<I1, I2> {
    /// Returns whether a mismatch in the two sizes of the two iterators was
    /// detected during iteration.
    ///
    /// This returns `true` if and only if, at some previous call to
    /// [`Iterator::next`] on this instance, one of the constituent iterators
    /// had a next element and the other did not.
    ///
    /// [`Iterator::next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
    pub(crate) fn has_size_mismatch(&self) -> bool {
        self.has_size_mismatch
    }

    /// Returns the number of elements in the left iterator.
    ///
    /// This iterates through the remainder of the left iterator if necessary in
    /// order to get the true number of elements. It therefore consumes `self`.
    pub(crate) fn left_size(mut self) -> usize {
        self.consumed_elements + self.left.by_ref().count()
    }
}

impl<I1: Iterator, I2: Iterator> Iterator for ZippedIterator<I1, I2> {
    type Item = (I1::Item, I2::Item);

    fn next(&mut self) -> Option<(I1::Item, I2::Item)> {
        match (self.left.next(), self.right.next()) {
            (Some(v1), Some(v2)) => {
                self.consumed_elements += 1;
                Some((v1, v2))
            }
            (Some(_), None) => {
                // Consumed elements counts only elements from self.left
                self.consumed_elements += 1;
                self.has_size_mismatch = true;
                None
            }
            (None, Some(_)) => {
                self.has_size_mismatch = true;
                None
            }
            (None, None) => None,
        }
    }
}
