#[allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{
        atomic::{Ordering::*, *},
        *,
    },
    thread::{self, sleep, Thread},
    time,
};

pub fn basics() {
    let numbers = vec![1, 2, 3];

    let t1 = thread::spawn(move || {
        for n in &numbers {
            println!("number: {n}");
        }
    });
    let t2 = thread::spawn(f);

    // let more_numbers = vec![0..=1000];
    let more_numbers = Vec::from_iter(0..=777);

    let t3 = thread::spawn(move || {
        let len = more_numbers.len();
        let sum = more_numbers.iter().sum::<usize>();
        sum / len
    });

    let nums_scoped = Vec::from_iter(0..15);

    // NOTE: scoped threads are still limited by the borrow checker, and thus you cannot have a mutable and immuatable borrow at the same time.
    thread::scope(|s| {
        s.spawn(|| {
            for n in &nums_scoped {
                println!("scoped n: {n}");
            }
        });
        s.spawn(|| {
            println!("nums_scoped len: {}", nums_scoped.len());
        });
    });

    // required to see any output without joining
    // sleep(time::Duration::from_secs(5));

    println!("Hello from the main thread.");

    t1.join().unwrap();
    t2.join().unwrap();
    let average = t3.join().unwrap();

    println!("average: {average}");
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
