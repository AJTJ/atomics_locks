use std::sync::atomic::{AtomicI32, Ordering};

pub fn fetch_add_example() {
    let a = AtomicI32::new(100);
    let b = a.fetch_add(23, Ordering::Relaxed);
    let c = a.load(Ordering::Relaxed);

    // b returns the a value
    assert_eq!(b, 100);

    // a is now 123
    assert_eq!(c, 123);
}
