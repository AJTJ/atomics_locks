use std::cell::Cell;

mod ch_1_basics;
mod ch_2_atomics;

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

fn main() {
    // ch_1_basics::basics();
    // ch_1_basics::undefined_behavior();

    // let a_val = Cell::new(8);
    // let b_val = Cell::new(20);
    // ch_1_basics::cell_usage(&a_val, &b_val);
    // ch_1_basics::cell_usage(&a_val, &a_val);
    // ch_1_basics::mutex_use();
    // ch_1_basics::thread_parking_queue();
    // ch_1_basics::condvar_usage();

    // ch_2_atomics::load_and_store::stop_flag();
    // ch_2_atomics::load_and_store::progress_reporting();
    // ch_2_atomics::lazy_init::get_x();
    ch_2_atomics::fetch_modify::fetch_add_example()
}
