use std::sync::atomic::{AtomicU32, Ordering};

// one prob: The 4,294,967,296th call will overflow the 32-bit integer
pub fn allocate_new_id() {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Ordering::Relaxed);
}

// panic once the id has gone over 1000
// problem here is that it can still be called by another thread, and increment the ID first before panicking.
// it will take a long tim, but it could still call it that many times and overflow
pub fn allocate_new_id_panic() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    assert!(id < 1000, "too many IDs");
    id
}

pub fn allocate_new_id_subtract() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Ordering::Relaxed);
        panic!("too many IDs");
    }
    id
}
