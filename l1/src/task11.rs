use std::collections::HashMap;

fn task(temperatures: Vec<f64>) {
    let mut intervals: HashMap<i32, Vec<f64>> = HashMap::new();

    for temp in temperatures {
        let interval_start = (temp / 10.0).floor() as i32 * 10;
        intervals.entry(interval_start).or_insert(Vec::new()).push(temp);
    }

    for (interval_start, temps) in intervals {
        println!("[{}, {}): {:?}", interval_start, interval_start + 10, temps);
    }
}


fn main() {
    let temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    task(temperatures)
}
