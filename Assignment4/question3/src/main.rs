use std::cell::Cell;

enum Level {
    Low,
    Medium,
    High
}

struct Task {
    id: Cell<u8>,
    level: Level
}

fn main() {
    let task = Task {
        id: Cell::new(10),
        level: Level::High
    };

    task.id.set(100);
    println!("Task with ID: {}", task.id.get());
}