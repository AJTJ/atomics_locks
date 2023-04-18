use std::{
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

use rand::Rng;
/*
 Some problems here are that we could briefly be reporting an innaccurate average, since the main thread can load the values after a thread has incremented num_done but before it has updated total_time.

 All three values could be placed inside a Mutex, slowing things down further.
*/
pub fn stats() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // four thread to process all 100 items, 25 each
        for t in 0..4 {
            s.spawn(|| {
                for i in 0..25 {
                    let start = Instant::now();
                    let mut rng = rand::thread_rng();
                    thread::sleep(Duration::from_millis(rng.gen_range(200..300) + 1));
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Ordering::Relaxed);
                    total_time.fetch_add(time_taken, Ordering::Relaxed);
                    max_time.fetch_max(time_taken, Ordering::Relaxed);
                }
            });
        }

        loop {
            let total_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            if n == 0 {
                println!("Working.. nothing done yet.");
            } else {
                println!(
                    "Working.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}
