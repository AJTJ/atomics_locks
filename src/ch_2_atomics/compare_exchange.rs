/*Example signature

impl AtomicI32 {
  pub fn compare_exchange(
      &self,
      expected: i32,
      new: i32,
      success_order: Ordering,
      failure_order: Ordering
  ) -> Result<i32, i32>;
}

Identical to this:

impl AtomicI32 {
    pub fn compare_exchange(&self, expected: i32, new: i32) -> Result<i32, i32> {
        // In reality, the load, comparison and store,
        // all happen as a single atomic operation.
        let v = self.load();
        if v == expected {
            // Value is as expected.
            // Replace it and report success.
            self.store(new);
            Ok(v)
        } else {
            // The value was not as expected.
            // Leave it untouched and report failure.
            Err(v)
        }
    }
}
*/

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use rand::Rng;

// fetch_add using compare_exchange
pub fn increment_compare_exchange(a: &AtomicU32) {
    let mut current = a.load(Ordering::Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            // if it is not the same as before, another thread chagned it in the moment since we loaded it.
            // we are given the new value that a has, and we try again with that value
            // if the value changes from A to B and then back to A again after the load operation, this is often fine, but can be a problem for atomic pointers. This is the ABA problem
            Err(v) => current = v,
        }
    }
}
// This is the correct, since we check and panic BEFORE modifying NEXT_ID
pub fn allocate_new_id_upper_bound() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Ordering::Relaxed);
    loop {
        assert!(id < 1000 /* or u32::MAX */, "too many IDs");
        match NEXT_ID.compare_exchange(id, id + 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}

// convenience method fetch_update
pub fn allocate_new_id_fetch_update() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
        .expect("too many IDs")
}

pub fn lazy_one_time_key_initialization() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Ordering::Relaxed);
    if key == 0 {
        let new_key = rand::thread_rng().gen::<u64>();
        match KEY.compare_exchange(0, new_key, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => new_key,
            // here, if we lost the race with another thread to gen a new key
            // we simply return the key that is in there
            Err(k) => k,
        }
    } else {
        key
    }
}
