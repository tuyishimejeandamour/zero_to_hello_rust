#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());

    print!("menoru size is: {}", mem::size_of_val(&p1));
    print!("menoru size is: {}", mem::size_of_val(&p2))
}
