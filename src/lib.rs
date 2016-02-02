//! SplitMut - a crate for retreiving multiple mutable values within the same collection
//!
//! # Example
//! ```
//! use std::collections::HashMap;
//! use splitmut::SplitMut;
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
//! ```

use std::ptr::null_mut;
use std::collections::{HashMap, BTreeMap, VecDeque};

#[inline]
fn to_ptr<V>(s: Option<&mut V>) -> *mut V { s.map(|q| q as *mut V).unwrap_or(null_mut()) }

fn check_ptr<V>(a: *mut V, b: *mut V) {
    if a == b && a != null_mut() { panic!("SplitMut called with identical keys!"); }
}

#[inline]
unsafe fn from_ptr<'q, V>(a: *mut V) -> Option<&'q mut V> {
    if a == null_mut() { None } else { Some(&mut *a) }
}

/// Just add `use splitmut::SplitMut;` to have these methods working on
/// mutable slices, Vec, VecDeque, HashMap and BTreeMap.
pub trait SplitMut<K, V> {
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
    /// 
    /// # Panic
    /// This function will panic if the two keys point to the same value.
    fn get2_mut(&mut self, k1: K, k2: K) -> (Option<&mut V>, Option<&mut V>) {
        let p1 = to_ptr(self.get1_mut(k1));
        let p2 = to_ptr(self.get1_mut(k2));
        check_ptr(p1, p2);
        unsafe { (from_ptr(p1), from_ptr(p2)) }
    }

    /// Returns three mutable references to three distinct values within
    /// the same collection.
    /// 
    /// # Panic
    /// This function will panic if any two keys point to the same value.
    fn get3_mut(&mut self, k1: K, k2: K, k3: K) -> (Option<&mut V>, Option<&mut V>, Option<&mut V>) {
        let p1 = to_ptr(self.get1_mut(k1));
        let p2 = to_ptr(self.get1_mut(k2));
        let p3 = to_ptr(self.get1_mut(k3));
        check_ptr(p1, p2);
        check_ptr(p1, p3);
        check_ptr(p2, p3);
        unsafe { (from_ptr(p1), from_ptr(p2), from_ptr(p3)) }
    }

    /// Returns four mutable references to four distinct values within
    /// the same collection.
    /// 
    /// # Panic
    /// This function will panic if any two keys point to the same value.
    fn get4_mut(&mut self, k1: K, k2: K, k3: K, k4: K) -> (Option<&mut V>, Option<&mut V>, Option<&mut V>, Option<&mut V>) {
        let p1 = to_ptr(self.get1_mut(k1));
        let p2 = to_ptr(self.get1_mut(k2));
        let p3 = to_ptr(self.get1_mut(k3));
        let p4 = to_ptr(self.get1_mut(k4));
        check_ptr(p1, p2);
        check_ptr(p1, p3);
        check_ptr(p2, p3);
        check_ptr(p1, p4);
        check_ptr(p2, p4);
        check_ptr(p3, p4);
        unsafe { (from_ptr(p1), from_ptr(p2), from_ptr(p3), from_ptr(p4)) }
    }

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

impl<'a, V> SplitMut<usize, V> for &'a mut [V] {
    #[inline]
    fn get1_mut(&mut self, k: usize) -> Option<&mut V> { self.get_mut(k) }
    #[inline]
    unsafe fn get1_unchecked_mut(&mut self, k: usize) -> &mut V { self.get_unchecked_mut(k) }
}

impl<'a, V> SplitMut<usize, V> for Vec<V> {
    #[inline]
    fn get1_mut(&mut self, k: usize) -> Option<&mut V> { self.get_mut(k) }
    #[inline]
    unsafe fn get1_unchecked_mut(&mut self, k: usize) -> &mut V { self.get_unchecked_mut(k) }
}

impl<'a, V> SplitMut<usize, V> for VecDeque<V> {
    #[inline]
    fn get1_mut(&mut self, k: usize) -> Option<&mut V> { self.get_mut(k) }
    #[inline]
    unsafe fn get1_unchecked_mut(&mut self, k: usize) -> &mut V { std::mem::transmute(self.get_mut(k)) }
}

impl<'a, K: std::hash::Hash + Eq, V> SplitMut<&'a K, V> for HashMap<K, V> {
    #[inline]
    fn get1_mut(&mut self, k: &'a K) -> Option<&mut V> { self.get_mut(k) }
    #[inline]
    unsafe fn get1_unchecked_mut(&mut self, k: &'a K) -> &mut V { std::mem::transmute(self.get_mut(k)) }
}

impl<'a, K: Ord, V> SplitMut<&'a K, V> for BTreeMap<K, V> {
    #[inline]
    fn get1_mut(&mut self, k: &'a K) -> Option<&mut V> { self.get_mut(k) }
    #[inline]
    unsafe fn get1_unchecked_mut(&mut self, k: &'a K) -> &mut V { std::mem::transmute(self.get_mut(k)) }
}

#[test]
#[should_panic]
fn hash_same() {
    let mut h = HashMap::new();
    h.insert(3u8, 5u16);
    h.get2_mut(&3, &3);
}

#[test]
fn hash_reg() {
    let mut h = HashMap::new();
    h.insert(3u8, 5u16);
    h.insert(4u8, 9u16);
    { let (a, b) = h.get2_mut(&3, &4);
      std::mem::swap(a.unwrap(), b.unwrap());
    }
    assert_eq!(h.get2_mut(&2, &2), (None, None));
    assert_eq!(unsafe { h.get2_unchecked_mut(&3, &4) }, (&mut 9u16, &mut 5u16));
    assert_eq!(h.get2_mut(&2, &3), (None, Some(&mut 9u16)));
}

#[test]
#[should_panic]
fn deque_same() {
    let mut h = VecDeque::new();
    h.push_front(5u16);
    h.get2_mut(0, 0);
}

#[test]
fn deque_reg() {
    let mut h = VecDeque::new();
    h.push_back(5u16);
    h.push_back(9u16);
    { let (a, b) = h.get2_mut(0, 1);
      std::mem::swap(a.unwrap(), b.unwrap());
    }
    assert_eq!(h.get2_mut(2, 2), (None, None));
    assert_eq!(unsafe { h.get2_unchecked_mut(0, 1) }, (&mut 9u16, &mut 5u16));
    assert_eq!(h.get2_mut(2, 0), (None, Some(&mut 9u16)));
}

#[test]
fn vec() {
    let mut h = vec!["Hello", "world", "!"];
    { let (a, b, c) = h.get3_mut(0, 1, 2);
      *c.unwrap() = "universe";
      std::mem::swap(a.unwrap(), b.unwrap());
    }
    assert_eq!(&*h, &["world", "Hello", "universe"]);

}
