// Copyright 2022 Google LLC
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

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

/// A trait to extract the size of a container.
///
/// The Rust standard library does not provide a trait to get the size of a
/// container. This is used for the `size(...)` matcher.
// Other implementations to consider: std::ops::Range.
pub trait HasSize {
    fn size(&self) -> usize;
}

macro_rules! impl_single {
    ($c:tt) => {
        impl<T> HasSize for $c<T> {
            fn size(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_single!(Vec);
impl_single!(VecDeque);
impl_single!(LinkedList);
impl_single!(HashSet);
impl_single!(BTreeSet);
impl_single!(BinaryHeap);

macro_rules! impl_pair {
    ($c:tt) => {
        impl<T, U> HasSize for $c<T, U> {
            fn size(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_pair!(HashMap);
impl_pair!(BTreeMap);

impl<T> HasSize for &[T] {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T, const N: usize> HasSize for [T; N] {
    fn size(&self) -> usize {
        N
    }
}

impl<T> HasSize for Box<[T]> {
    fn size(&self) -> usize {
        self.len()
    }
}
