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
use std::{time::Duration, vec};

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

// https://marabos.nl/atomics/basics.html#lifetime-of-mutexguard
// Lifetime of mutex guard

pub fn mutex_guard_lifetime() {
    let list = Mutex::new(vec![0]);

    list.lock().unwrap().push(1);

    // Here the lock remains intact for the entire duration of the long_process_fn
    if let Some(item) = list.lock().unwrap().pop() {
        // long_process_fn(item)
        println!("the item: {item}");
    };

    // the lock is dropped before the long_process_fn, because
    // the condition of a regular if statement is a always a plain boolean
    // which does not borrow anything
    if list.lock().unwrap().pop() == Some(1) {
        // long_process_fn
    }

    let item = list.lock().unwrap().pop();
    // The guard is dropped here before the if let statement
    if let Some(item) = item {
        // long_process_fn(item)
    }
}

// https://marabos.nl/atomics/basics.html#reader-writer-lock
// A mutex only allows exclusive access (&mut T)
// An RwLock can allow for a shared reference (&T)
// Essentialy the multi-threaded version of RefCell

// https://marabos.nl/atomics/basics.html#waiting
// Thread Parking
// a parked thread doesn't consume CPU cycles

/*
- A couple notes
    - It would still be correct without parking.
    - threads can have "spurious wakeups"
    - A call to "unpark" does not get lost, and rather causes the next "park" request to "unpark", but "unpark" requests do not stack.
 */
pub fn thread_parking_queue() {
    let queue: Mutex<VecDeque<isize>> = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        // consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });
        // producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    })
}

// The above example begins to break down with multiple consumers
// the producer thread has no way of knowing which consumer is actually waiting and which should be woken up.
// a more sophisticated approach is required

// https://marabos.nl/atomics/basics.html#condvar

pub fn condvar_usage() {
    let queue: Mutex<VecDeque<isize>> = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = not_empty.wait(q).unwrap();
                }
            };
            drop(q);
            dbg!(item);
        });

        for i in 0..25 {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    })
}

// NOTE This probably doesn't make sense. Better to have a hashmap of things, or something else.
pub fn another_condvar_usage() {
    let queue: Mutex<VecDeque<Foo>> = Mutex::new(VecDeque::new());

    enum Foo {
        Apple(String),
        Orange(String),
    }
    let containes_orange = Condvar::new();
    let contains_apple = Condvar::new();

    thread::scope(|s| {
        // Orange thread
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            for item in q.iter_mut() {}
            // loop over the queue and pop all the oranges
            // NOTE This probably doesn't make sense. Better to have a hashmap of things, or something else.
        });

        // Apple thread
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            // loop over the queue and pop all the apples
        });
    });

    for i in 0..25 {
        if i % 3 == 0 {
            queue
                .lock()
                .unwrap()
                .push_back(Foo::Apple(format!("Yum apple! - {}", i).to_string()));
            contains_apple.notify_one();
            thread::sleep(Duration::from_secs(1));
        } else {
            queue
                .lock()
                .unwrap()
                .push_back(Foo::Apple(format!("Orangy! - {}", i).to_string()));
            containes_orange.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    }
}
