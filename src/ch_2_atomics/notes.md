### GENERAL
- In Rust
  - In rust atomic operations are available as methods on the standard atomic types that live in std::sync::atomic.
    - They are prefixed with `Atomic`
    - Their availability is dependent on the hardware and architecture.
    - But almost all atomic types up to the size of a pointer are available.
    - Atomic primitives allow mutation through a shared ref ie: `&AtomicU8`
    - Each atomic operation takes an argument of `std::sync::atomic::Ordering` which determines what guarantees we get with the relative order of operations.
      - The simplest variant is `Relaxed` which maintains consistency on a single atomic variable, but does not guarantee any consistency in the order of operations.
      - With `Relaxed`, two threads might see different operations happening at different times.

