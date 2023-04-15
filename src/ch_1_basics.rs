use std::time::Duration;
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

// https://marabos.nl/atomics/basics.html#undefined-behavior
/*
- "Unsafe" code simply means that the compiler hasn't validated the code
- Undefined behavior should be avoided at all costs
 */

pub fn undefined_behavior() {
    let a = [123, 456, 789];
    let b = unsafe { a.get_unchecked(11) };
    println!("this is b, and could be anything: {b}");
}

// https://marabos.nl/atomics/basics.html#interior-mutability

// Cell
// - A single-thread-safe interior mutability container.
// - Can't borrow the contents
pub fn cell_usage(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("{before} != {after}");
    } else {
        println!("{before} == {after}");
    }
}

// RefCell
// Single-thread-safe interior mutability container that allows borrowing
pub fn ref_cell_usage(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}

// Concurrency Types
// https://marabos.nl/atomics/basics.html#mutex-and-rwlock
// Mutex and RwLock

// Atomics

// https://marabos.nl/atomics/basics.html#unsafecell
// UnsafeCell<T>
// primitive building block that all interior mutability containers are built upon
// could be meaningfully used in unsafe block

// https://marabos.nl/atomics/basics.html#thread-safety
// Send and Sync -> See notes

pub fn send_and_sync() {
    // X is not Sync here because Cell is not Sync
    struct X {
        handle: i32,
        not_sync: PhantomData<Cell<()>>,
    }

    let a = Rc::new(123);
    // Rc is not Send, therefore the compiler won't allow the following
    // thread::spawn(move || dbg!(a));
}

// Mutexes
// https://marabos.nl/atomics/basics.html#mutexes
/*
- Short for "mutual exclusion"
- The job of a mutex is to provide exclusive access to some data by temporarily blocking other threads' access to the data at the same time.
- unlocking is done by dropping the `MutexGuard` which is returned from the lock()
 */

pub fn mutex_use() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                println!("thread STARTED id: {:?}", thread::current().id());
                // because of the mutex, the increments of 100, are single, indivisible atomic operations
                let mut guard = n.lock().unwrap();

                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard);
                thread::sleep(Duration::from_secs(1));
                println!("thread DROPPED id: {:?}", thread::current().id());
            });
        }
    });

    println!("mutex_use n: {}", n.lock().unwrap());

    assert_eq!(n.into_inner().unwrap(), 1000);
}
