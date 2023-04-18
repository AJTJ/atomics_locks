use std::{
    io::stdin,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread,
    time::Duration,
};

pub fn stop_flag() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // spawn a thread to do the work.
    let background_thread = thread::spawn(|| {
        while !STOP.load(Ordering::Relaxed) {
            println!("background thread working");
            thread::sleep(Duration::from_secs(1));
        }
    });

    for line in stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    STOP.store(true, Ordering::Relaxed);

    background_thread.join().unwrap();
}

pub fn progress_reporting() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        // background thread to process all 100 items

        s.spawn(|| {
            for i in 0..100 {
                // presuming that the processing takes a bunch of time
                thread::sleep(Duration::from_millis(75));
                num_done.store(i + 1, Ordering::Relaxed);
                if i == 99 {
                    main_thread.unpark();
                }
            }
        });

        // why can't this be put outside the thread::scope closure?
        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            };
            println!("Working.. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
            // thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done");
}

// https://marabos.nl/atomics/atomics.html#example-progress-reporting-from-multiple-threads

pub fn progress_reporting_multiple_threads() {
    let num_done = &AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                thread::sleep(Duration::from_millis(75));
                for i in 0..25 {
                    // simulate work being done
                    println!("thread: {t}, i: {i}");
                    thread::sleep(Duration::from_millis(75));
                    num_done.fetch_add(1, Ordering::Relaxed);
                }
            });
        }

        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("processed {}/100", n);
            thread::park_timeout(Duration::from_secs(1));
        }
    })
}
