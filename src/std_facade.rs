//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module provides #[cfg(..)]ed type aliases over features.

#[macro_export]
macro_rules! multiplex_alloc {
    ($($alloc: path, $std: path),*) => {
        $(
            #[cfg(all(feature = "alloc", not(feature = "std")))]
            pub(crate) use $alloc;
            #[cfg(feature = "std")]
            pub(crate) use $std;
        )*
    };
}

#[macro_export]
macro_rules! multiplex_core {
    ($($core: path, $std: path),*) => {
        $(
            #[cfg(not(feature = "std"))]
            pub(crate) use $core;
            #[cfg(feature = "std")]
            pub(crate) use $std;
        )*
    };
}

multiplex_alloc! {
    alloc::borrow::Cow, ::std::borrow::Cow,
    alloc::borrow::ToOwned, ::std::borrow::ToOwned,
    alloc::boxed::Box, ::std::boxed::Box,
    alloc::String, ::std::string::String,
    alloc::string, ::std::string,
    alloc::arc::Arc, ::std::sync::Arc,
    alloc::rc::Rc, ::std::rc::Rc,
    alloc::Vec, ::std::vec::Vec,
    alloc::vec, ::std::vec,
    alloc::VecDeque, std::collections::VecDeque,
    alloc::vec_deque, std::collections::vec_deque,
    alloc::BinaryHeap, ::std::collections::BinaryHeap,
    alloc::binary_heap, ::std::collections::binary_heap,
    alloc::LinkedList, ::std::collections::LinkedList,
    alloc::linked_list, ::std::collections::linked_list,
    alloc::BTreeSet, ::std::collections::BTreeSet,
    alloc::BTreeMap, ::std::collections::BTreeMap,
    alloc::btree_map, ::std::collections::btree_map,
    alloc::btree_set, ::std::collections::btree_set
}

#[cfg(feature = "std")]
multiplex_alloc! {
    hashmap_core::HashMap, ::std::collections::HashMap,
    hashmap_core::HashSet, ::std::collections::HashSet
}

//#[cfg(not(feature = "std"))]
//pub(crate) use hashmap_core::map as hash_map;
#[cfg(feature = "std")]
pub(crate) use ::std::collections::hash_map;
//#[cfg(not(feature = "std"))]
//pub(crate) use hashmap_core::set as hash_set;
#[cfg(feature = "std")]
pub(crate) use ::std::collections::hash_set;

multiplex_core! {
    core::fmt, ::std::fmt,
    core::cell::Cell, ::std::cell::Cell
}
