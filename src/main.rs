use std::cell::Cell;

mod ch_1_basics;

fn main() {
    // ch_1_basics::basics();
    // ch_1_basics::undefined_behavior();

    let a_val = Cell::new(8);
    let b_val = Cell::new(20);
    ch_1_basics::cell_usage(&a_val, &b_val);
    ch_1_basics::cell_usage(&a_val, &a_val);

    ch_1_basics::mutex_use();
}
