fn task(num: i64, pos: u32, bit: bool) -> i64 {
    if bit {
        num | (1 << pos)
    } else {
        num & (1 << pos)
    }
}

fn main() {
    let num: i64 = 0b1010;
    let pos: u32 = 0;

    let result1 = task(num, pos, true);
    let result2 = task(num, pos, false);

    println!("Значение после установки {}-го бита в 1: {:b}", pos, result1);
    println!("Значение после установки {}-го бита в 0: {:b}", pos, result2);
}