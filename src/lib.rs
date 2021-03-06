//! SplitMut - a crate for safely retrieving multiple mutable values within the same collection.
//!
//! `get2_mut`, `get3_mut` and `get4_mut` return a tuple or 2, 3 or 4 values, each one of them being
//! one of `Ok(&mut V)`, `Err(SplitMutError::NoValue)` in case there was no value for the key (i e, when
//! your usual `get_mut` would have returned `None`), or `Err(SplitMutError::SameValue)` in case the same
//! value has already been returned earlier in the tuple. 
//!
//! If you need more than four values, you can use `get_muts` or `get_mut_iter` to get as many mutable
//! values as you like.
//!
//! # Example
//! ```
//! use std::collections::HashMap;
//! use splitmut::{SplitMut, SplitMutError};
//!
//! // Create a hashmap
//! let mut h = HashMap::new();
//! h.insert(1, "Hello");
//! h.insert(2, "world");
//!
//! // Swap two values easily
//! {
//!     let (m1, m2) = h.get2_mut(&1, &2);
//!     std::mem::swap(m1.unwrap(), m2.unwrap());
//! }
//! assert_eq!(h.get(&1), Some(&"world"));
//! assert_eq!(h.get(&2), Some(&"Hello"));
//!
//! // Show error handling
//! let (m0, m1a, m1b) = h.get3_mut(&0, &1, &1);
//! // No value for the key "0"
//! assert_eq!(m0, Err(SplitMutError::NoValue));
//! // First value for the key "1" is returned successfully
//! assert_eq!(m1a, Ok(&mut "world"));
//! // Second value for the key "1" returns an error
//! assert_eq!(m1b, Err(SplitMutError::SameValue));
//! ```
//!

#![warn(missing_docs)]

use std::collections::{HashMap, BTreeMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::{hash, borrow};

/// Error returned from get*_mut functions.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub enum SplitMutError {
    /// No value was found for the specified key (like when get_mut would return None)
    NoValue,
    /// The same value has already been returned (earlier in the same tuple)
    SameValue,
}

impl std::error::Error for SplitMutError {
    fn description(&self) -> &'static str {
         match self {
              &SplitMutError::NoValue => "No value",
              &SplitMutError::SameValue => "Duplicate values",
         }
    }
}

impl std::fmt::Display for SplitMutError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
         use std::error::Error;
         f.write_str(self.description())
    }
}

// Used internally.
type R<V> = Result<*mut V, SplitMutError>;

#[inline]
fn to_r<V>(s: Option<&mut V>) -> R<V> {
    s.map(|s| s as *mut V).ok_or(SplitMutError::NoValue)
}

#[inline]
fn check_r<V>(a: &R<V>, b: R<V>) -> R<V> {
    match (a, &b) {
        (&Ok(ref aa), &Ok(ref bb)) => if aa == bb { return Err(SplitMutError::SameValue) },
        _ => {},
    }
    b
}

#[inline]
unsafe fn from_r<'a, V>(a: R<V>) -> Result<&'a mut V, SplitMutError> { a.map(|aa| &mut *aa) } 


/// Just add `use splitmut::SplitMut;` to have these methods working on
/// mutable slices, Vec, VecDeque, HashMap and BTreeMap.
///
/// In case you want to implement `SplitMut` for your own collection, just
/// implement `get1_mut` and `get1_unchecked_mut` and the other methods will
/// be provided for you. If you do so, you must make sure that these functions
/// do not mutate your collection in ways that would invalidate previously returned
/// values from `get1_mut` and `get1_unchecked_mut`.
pub unsafe trait SplitMut<K, V> {
    /// Wrapper for get_mut, used internally.
    fn get1_mut(&mut self, k1: K) -> Option<&mut V>;
    /// Wrapper for get_unchecked_mut, used internally.
    ///
    /// # Undefined behaviour
    /// It is undefined behaviour to call this with a key that does not correspond to a value.
    /// You have been warned.
    unsafe fn get1_unchecked_mut(&mut self, k1: K) -> &mut V;

    /// Returns two mutable references to two distinct values within
    /// the same collection.
    fn get2_mut(&mut self, k1: K, k2: K) -> (Result<&mut V, SplitMutError>, Result<&mut V, SplitMutError>) {
        let p1 = to_r(self.get1_mut(k1));
        let p2 = to_r(self.get1_mut(k2));
        let p2 = check_r(&p1, p2);
        unsafe { (from_r(p1), from_r(p2)) }
    }

    /// Returns three mutable references to three distinct values within
    /// the same collection.
    fn get3_mut(&mut self, k1: K, k2: K, k3: K) -> (Result<&mut V, SplitMutError>, 
        Result<&mut V, SplitMutError>, Result<&mut V, SplitMutError>) {

        let p1 = to_r(self.get1_mut(k1));
        let p2 = to_r(self.get1_mut(k2));
        let p3 = to_r(self.get1_mut(k3));
        let p2 = check_r(&p1, p2);
        let p3 = check_r(&p1, p3);
        let p3 = check_r(&p2, p3);
        unsafe { (from_r(p1), from_r(p2), from_r(p3)) }
    }

    /// Returns four mutable references to four distinct values within
    /// the same collection.
    fn get4_mut(&mut self, k1: K, k2: K, k3: K, k4: K) -> (Result<&mut V, SplitMutError>,
        Result<&mut V, SplitMutError>, Result<&mut V, SplitMutError>, Result<&mut V, SplitMutError>) {
        let p1 = to_r(self.get1_mut(k1));
        let p2 = to_r(self.get1_mut(k2));
        let p3 = to_r(self.get1_mut(k3));
        let p4 = to_r(self.get1_mut(k4));
        let p2 = check_r(&p1, p2);
        let p3 = check_r(&p1, p3);
        let p3 = check_r(&p2, p3);
        let p4 = check_r(&p1, p4);
        let p4 = check_r(&p2, p4);
        let p4 = check_r(&p3, p4);
        unsafe { (from_r(p1), from_r(p2), from_r(p3), from_r(p4)) }
    }

    /// Returns any number mutable references to distinct values within
    /// the same collection. A HashSet is used internally to keep track
    /// of values already returned.
    ///
    /// # Example
    /// ```
    /// use splitmut::SplitMut;
    ///
    /// let mut h = vec!["Hello", "world", "!"];
    /// let mut z = h.get_muts();
    /// let a = z.at(0);
    /// let b = z.at(1);
    /// assert_eq!(a, Ok(&mut "Hello"));
    /// assert_eq!(b, Ok(&mut "world"));
    /// ```
    fn get_muts(&mut self) -> GetMuts<K, V, Self> { GetMuts(self, HashSet::new(), PhantomData) }

    /// Returns an iterator adapter that maps from a K to a Result<V, SplitMutError>.
    /// A HashSet is used internally to keep track of values already returned.
    ///
    /// # Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use splitmut::{SplitMut, SplitMutError};
    ///
    /// let mut h = BTreeMap::new();
    /// h.insert(String::from("borrow"), 1);   
    /// h.insert(String::from("me"), 2);
    /// let slice = ["me", "borrow", "me"];
    /// let z: Vec<_> = h.get_mut_iter(slice.into_iter().map(|&k| k)).collect();
    /// assert_eq!(&*z, [Ok(&mut 2), Ok(&mut 1), Err(SplitMutError::SameValue)]);
    /// ```
    fn get_mut_iter<I: Iterator<Item=K>>(&mut self, i: I) -> GetMutIter<K, V, Self, I> { GetMutIter(self.get_muts(), i) }

    /// Returns two mutable references to two distinct values within
    /// the same collection.
    /// 
    /// # Undefined behaviour
    /// It is undefined behaviour to call this with a key that does not
    /// correspond to a value, or with keys pointing to the same value.
    /// You have been warned.
    unsafe fn get2_unchecked_mut(&mut self, k1: K, k2: K) -> (&mut V, &mut V) {
        let p2 = self.get1_unchecked_mut(k2) as *mut V;
        (self.get1_unchecked_mut(k1), &mut *p2)
    }

    /// Returns three mutable references to three distinct values within
    /// the same collection.
    /// 
    /// # Undefined behaviour
    /// It is undefined behaviour to call this with a key that does not
    /// correspond to a value, or with any two keys pointing to the same value.
    /// You have been warned.
    unsafe fn get3_unchecked_mut(&mut self, k1: K, k2: K, k3: K) -> (&mut V, &mut V, &mut V) {
        let p2 = self.get1_unchecked_mut(k2) as *mut V;
        let p3 = self.get1_unchecked_mut(k3) as *mut V;
        (self.get1_unchecked_mut(k1), &mut *p2, &mut *p3)
    }

    /// Returns four mutable references to four distinct values within
    /// the same collection.
    /// 
    /// # Undefined behaviour
    /// It is undefined behaviour to call this with a key that does not
    /// correspond to a value, or with any two keys pointing to the same value.
    /// You have been warned.
    unsafe fn get4_unchecked_mut(&mut self, k1: K, k2: K, k3: K, k4: K) -> (&mut V, &mut V, &mut V, &mut V) {
        let p2 = self.get1_unchecked_mut(k2) as *mut V;
        let p3 = self.get1_unchecked_mut(k3) as *mut V;
        let p4 = self.get1_unchecked_mut(k4) as *mut V;
        (self.get1_unchecked_mut(k1), &mut *p2, &mut *p3, &mut *p4)
    }
}

/// Wrapper struct for the get_muts function. 
pub struct GetMuts<'a, K, V, A: 'a + SplitMut<K, V> + ?Sized>(&'a mut A, HashSet<*mut V>, PhantomData<*const K>);

impl<'a, K, V, A: 'a + SplitMut<K, V> + ?Sized> GetMuts<'a, K, V, A> {
    /// Returns a mutable reference with the same lifetime as the
    /// original collection, i e, it allows several distinct mutable references to
    /// be alive simultaneously.
    ///
    /// It returnes an error if the value does not exist or has already been returned.
    pub fn at(&mut self, k: K) -> Result<&'a mut V, SplitMutError> {
        let p = try!(to_r(self.0.get1_mut(k)));
        if !self.1.insert(p) { return Err(SplitMutError::SameValue) };
        Ok(unsafe { &mut *p })
    }
}


/// Wrapper struct for the get_mut_iter function. 
pub struct GetMutIter<'a, K, V, A: 'a + SplitMut<K, V> + ?Sized, I>(GetMuts<'a, K, V, A>, I);

impl<'a, K, V: 'a, A: 'a + SplitMut<K, V> + ?Sized, I: Iterator<Item=K>> Iterator for GetMutIter<'a, K, V, A, I> {
    type Item = Result<&'a mut V, SplitMutError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|k| self.0.at(k))
    }
}


unsafe impl<'a, V> SplitMut<usize, V> for &'a mut [V] {
    #[inline]
    fn get1_mut(&mut self, k: usize) -> Option<&mut V> { self.get_mut(k) }
    #[inline]
    unsafe fn get1_unchecked_mut(&mut self, k: usize) -> &mut V { self.get_unchecked_mut(k) }
}

unsafe impl<'a, V> SplitMut<usize, V> for Vec<V> {
    #[inline]
    fn get1_mut(&mut self, k: usize) -> Option<&mut V> { self.get_mut(k) }
    #[inline]
    unsafe fn get1_unchecked_mut(&mut self, k: usize) -> &mut V { self.get_unchecked_mut(k) }
}

unsafe impl<'a, V> SplitMut<usize, V> for VecDeque<V> {
    #[inline]
    fn get1_mut(&mut self, k: usize) -> Option<&mut V> { self.get_mut(k) }
    #[inline]
    unsafe fn get1_unchecked_mut(&mut self, k: usize) -> &mut V { std::mem::transmute(self.get_mut(k)) }
}

unsafe impl<'a, K: hash::Hash + Eq + borrow::Borrow<Q>, Q: hash::Hash + Eq + ?Sized, V, S: hash::BuildHasher> SplitMut<&'a Q, V> for HashMap<K, V, S> {
    #[inline]
    fn get1_mut(&mut self, k: &'a Q) -> Option<&mut V> { self.get_mut(k) }
    #[inline]
    unsafe fn get1_unchecked_mut(&mut self, k: &'a Q) -> &mut V { std::mem::transmute(self.get_mut(k)) }
}

unsafe impl<'a, K: Ord + borrow::Borrow<Q>, Q: Ord + ?Sized, V> SplitMut<&'a Q, V> for BTreeMap<K, V> {
    #[inline]
    fn get1_mut(&mut self, k: &'a Q) -> Option<&mut V> { self.get_mut(k) }
    #[inline]
    unsafe fn get1_unchecked_mut(&mut self, k: &'a Q) -> &mut V { std::mem::transmute(self.get_mut(k)) }
}

#[test]
fn hash_same() {
    let mut h = HashMap::new();
    h.insert(3u8, 5u16);
    assert_eq!(h.get2_mut(&3, &3), (Ok(&mut 5u16), Err(SplitMutError::SameValue)));
}

#[test]
fn hash_reg() {
    let mut h = HashMap::new();
    h.insert(3u8, 5u16);
    h.insert(4u8, 9u16);
    { let (a, b) = h.get2_mut(&3, &4);
      std::mem::swap(a.unwrap(), b.unwrap());
    }
    assert_eq!(h.get2_mut(&2, &2), (Err(SplitMutError::NoValue), Err(SplitMutError::NoValue)));
    assert_eq!(unsafe { h.get2_unchecked_mut(&3, &4) }, (&mut 9u16, &mut 5u16));
    assert_eq!(h.get2_mut(&2, &3), (Err(SplitMutError::NoValue), Ok(&mut 9u16)));
}

#[test]
fn tree_borrow() {
    let mut h = BTreeMap::new();
    h.insert(String::from("borrow"), 1);   
    h.insert(String::from("me"), 2);
    let slice = ["me", "borrow", "me"];
    let z: Vec<_> = h.get_mut_iter(slice.into_iter().map(|&k| k)).collect();
    assert_eq!(&*z, [Ok(&mut 2), Ok(&mut 1), Err(SplitMutError::SameValue)]);
}

#[test]
fn deque_same() {
    let mut h = VecDeque::new();
    h.push_front(5u16);
    assert_eq!(h.get2_mut(0, 0), (Ok(&mut 5u16), Err(SplitMutError::SameValue)));
}

#[test]
fn deque_reg() {
    let mut h = VecDeque::new();
    h.push_back(5u16);
    h.push_back(9u16);
    { let (a, b) = h.get2_mut(0, 1);
      std::mem::swap(a.unwrap(), b.unwrap());
    }
    assert_eq!(h.get2_mut(2, 2), (Err(SplitMutError::NoValue), Err(SplitMutError::NoValue)));
    assert_eq!(unsafe { h.get2_unchecked_mut(0, 1) }, (&mut 9u16, &mut 5u16));
    assert_eq!(h.get2_mut(2, 0), (Err(SplitMutError::NoValue), Ok(&mut 9u16)));
}

#[test]
fn vec() {
    let mut h = vec!["Hello", "world", "!"];
    { let (a, b, c) = h.get3_mut(0, 1, 2);
      *c.unwrap() = "universe";
      std::mem::swap(a.unwrap(), b.unwrap());
    }
    assert_eq!(&*h, &["world", "Hello", "universe"]);

    {
        let mut z = h.get_muts();
        let a = z.at(0);
        let b = z.at(1);
        assert_eq!(a, Ok(&mut "world"));
        assert_eq!(b, Ok(&mut "Hello"));
        std::mem::swap(a.unwrap(), b.unwrap());
        assert_eq!(z.at(0), Err(SplitMutError::SameValue));
        assert_eq!(z.at(3), Err(SplitMutError::NoValue));
    }
    assert_eq!(&*h, &["Hello", "world", "universe"]);
}
