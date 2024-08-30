use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use dashmap::DashMap;

/// Mutex с HashMap
fn task1() {
    let map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for i in 0..=10 {
        let map = map.clone();
        let handle = thread::spawn(move || {
            map.lock().unwrap().insert(i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let map = map.lock().unwrap();
    println!("{:?}", map);
}

/// DashMap
fn task2() {
    let map = Arc::new(DashMap::new());
    let mut handles = Vec::new();

    for i in 0..=10 {
        let map = map.clone();
        let handle = thread::spawn(move || {
            map.insert(i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", map);
}

fn main() {
    task1();
    task2();
}
