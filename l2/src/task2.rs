/// Состояние распакованной строки
#[derive(Debug, PartialEq)]
enum UnpackingState {
    Unprocessed(String),
    Ok(String),
    Err(String),
}

#[derive(Debug)]
struct UnpackingString {
    original_string: &'static str,
    unpacking_string: UnpackingState,
}

impl UnpackingString {
    /// Конструктор
    pub fn new(string: &'static str) -> Self {
        UnpackingString {
            original_string: string,
            unpacking_string: UnpackingState::Unprocessed(string.to_string()),
        }
    }

    /// Распаковка обычной последовательности
    fn standard_sequence(&mut self) -> UnpackingState {
        let mut chars = self.original_string.chars().peekable();
        let mut result = String::with_capacity(self.original_string.len());

        while let Some(ch) = chars.next() {
            // Если символ - цифра, но он идет в начале строки или после другой цифры - возвращаем ошибку
            if ch.is_digit(10) {
                return UnpackingState::Err(self.original_string.to_string());
            }
            result.push(ch);

            if let Some(next_ch) = chars.peek() {
                if next_ch.is_digit(10) {
                    // Если следующий символ - цифра, то повторяем текущий символ соответствующее количество раз
                    let count = next_ch.to_digit(10).unwrap();
                    result.push_str(ch.to_string().repeat((count - 1) as usize).as_str());
                    chars.next(); // Пропускаем цифру
                }
            }
        }

        UnpackingState::Ok(result)
    }

    /// Распаковка escape последовательности
    fn escape_sequence(&mut self) -> UnpackingState {
       todo!()
    }

    /// Распаковка строки
    pub fn unpacking(&mut self) {
        let result = if !self.original_string.contains('\\') {
            self.standard_sequence()
        } else {
            self.escape_sequence()
        };

        self.unpacking_string = result
    }
}


fn main() {
    let mut s = UnpackingString::new(r"aaaabccddddde");
    s.unpacking();
    println!("{:?}", s.unpacking_string)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn standard_sequence_test() {
        let arg_result = HashMap::from([
            ("a4bc2d5e", UnpackingState::Ok("aaaabccddddde".to_string())),
            ("abcd", UnpackingState::Ok("abcd".to_string())),
            ("45", UnpackingState::Err("45".to_string())),
            ("", UnpackingState::Ok("".to_string()))
        ]);

        for (arg, result) in arg_result {
            let mut unpacking_string = UnpackingString::new(arg);
            unpacking_string.unpacking();
            assert_eq!(unpacking_string.unpacking_string, result);
        }
    }

    // #[test]
    // fn escape_sequence_test() {
    //     let arg_result = HashMap::from([
    //         (r"qwe\4\5", "qwe45 (*)"),
    //         (r"qwe\45", "qwe44444 (*)"),
    //         (r"qwe\\5", r"qwe\\\\\ (*)")
    //     ]);
    // }
}