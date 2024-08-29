use std::fmt::{Debug, Display};
use std::cmp::PartialOrd;
use num::{CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};

struct ManyOperations<T> {
    nums: (T, T),
    mul: Option<T>,
    div: Option<T>,
    add: Option<T>,
    sub: Option<T>,
}

impl<T> ManyOperations<T>
where
    T: CheckedAdd<Output=T> + CheckedSub<Output=T> +
    CheckedMul<Output=T> + CheckedDiv<Output=T>
    + Debug + Display + PartialOrd,
{
    fn new(a: T, b: T) -> Self {
        let mul = a.checked_mul(&b);
        let div = a.checked_div(&b);
        let add = a.checked_add(&b);
        let sub = a.checked_sub(&b);

        ManyOperations { nums: (a, b), mul, div, add, sub }
    }

    fn check_result(&self) {
        self.print_result("умножения", &self.mul);
        self.print_result("деления", &self.div);
        self.print_result("сложения", &self.add);
        self.print_result("вычитания", &self.sub);
    }

    fn print_result(&self, operation: &str, result: &Option<T>) {
        match result {
            Some(num) => println!("Результат {} числа {} на {} равен {:?}", operation, self.nums.0, self.nums.1, num),
            None => println!("{} чисел привело к переполнению или ошибке", operation),
        }
    }
}


fn main() {
    let a = 100u8;
    let b = 50u8;

    let result = ManyOperations::new(a, b);
    result.check_result()
}
