use std::sync::{
    atomic::{AtomicU64, Ordering},
    Once, OnceLock,
};
/*
  The problem with this is that it another thread can call get_x()
  and then can cause a "Race"
  where one thread overwrites the value of X
  Not a data race, which is undefined behavior
  But still a "race" with an unpredictable winner.

  A possible solution is using a condvar or parking,
  but that is complex
*/
pub fn get_x() -> u64 {
    // presuming that it is non-0
    // to be calculated, but expected to be constant
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Ordering::Relaxed);

    if x == 0 {
        // do some calcuations
        x = 12;
        X.store(x, Ordering::Relaxed);
    }

    x
}

pub fn get_x_once() -> u64 {
    static START: Once = Once::new();
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Ordering::Relaxed);

    START.call_once(|| {
        // do some expensive calculation to calc the value
        x = 12;
        X.store(12, Ordering::Relaxed);
    });

    x
}

// much clearer and simpler
pub fn get_x_once_lock() -> u64 {
    static X: OnceLock<u64> = OnceLock::new();

    *X.get_or_init(|| {
        // expensive calcs
        12
    })
}
