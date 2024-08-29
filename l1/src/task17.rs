use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counter(AtomicUsize);

impl Counter {
    const fn new() -> Self {
        Counter(AtomicUsize::new(0))
    }

    fn increment(&self) {
        // Не до конца понимаю, какой тип упорядочивания памяти лучше выбрать,
        // поэтому сориентировался на реализацию из Arc
        self.0.fetch_add(1, Ordering::Release);
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        println!("{:?}", self.0)
    }
}

fn main() {
    let counter = Arc::new(Counter::new());
    let mut handles = Vec::new();

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(100_000, counter.0.load(Ordering::Relaxed))
}