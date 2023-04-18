use std::sync::atomic::{AtomicI32, Ordering};

/*
    https://marabos.nl/atomics/atomics.html#fetch-and-modify-operations
   function signature of AtomicI32

   impl AtomicI32 {
       pub fn fetch_add(&self, v: i32, ordering: Ordering) -> i32;
       pub fn fetch_sub(&self, v: i32, ordering: Ordering) -> i32;
       pub fn fetch_or(&self, v: i32, ordering: Ordering) -> i32;
       pub fn fetch_and(&self, v: i32, ordering: Ordering) -> i32;
       pub fn fetch_nand(&self, v: i32, ordering: Ordering) -> i32;
       pub fn fetch_xor(&self, v: i32, ordering: Ordering) -> i32;
       pub fn fetch_max(&self, v: i32, ordering: Ordering) -> i32;
       pub fn fetch_min(&self, v: i32, ordering: Ordering) -> i32;
       pub fn swap(&self, v: i32, ordering: Ordering) -> i32; // "fetch_store"
   }

   NOTE: fetch_add and fetch_sub implement wrapping behavior

*/

pub fn fetch_add_example() {
    let a = AtomicI32::new(100);
    let b = a.fetch_add(23, Ordering::Relaxed);
    let c = a.load(Ordering::Relaxed);

    // b returns the a value
    assert_eq!(b, 100);

    // a is now 123
    assert_eq!(c, 123);
}
