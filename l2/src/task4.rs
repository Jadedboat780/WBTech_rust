use std::collections::HashMap;

/// Поиск анаграмм в массиве слов
fn find_anagrams(words: &[&str]) -> HashMap<String, Vec<String>> {
    // Словарь для хранения групп анаграмм
    // Ключ - первое слово из анаграммы
    // Значение - массив слов
    let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();

    // Итерация по переданному набору словам
    for &word in words {
        let word = word.to_lowercase();  // приведение слова к нижнему регистру

        // Преобразование слова в вектор символов, чтобы его можно было отсортировать
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();

        // Преобразование вектора символов обратно в строку
        let sorted_word = chars.into_iter().collect();

        // Словарь анаграмм, где ключ - первое слово анаграммы
        // Если ключ уже существует - добавляется слово в нужную группу
        // Если ключа нет - создаётся новая группа
        anagrams
            .entry(sorted_word)
            .or_insert_with(Vec::new) // инициализируем новой группы при отсутствии ключа
            .push(word); // добавление текущего слова в группу
    }

    anagrams
        .into_iter()
        .filter_map(|(_, mut anagram_group)| {
            // Фильтрация групп больше одного слова
            if anagram_group.len() > 1 {
                anagram_group.sort(); // сортировка слов в алфавитном порядке
                Some((anagram_group[0].clone(), anagram_group))
            } else {
                // Если группа содержит только одно слово, то оно не включается в результат
                None
            }
        })
        .collect()
}

fn main() {
    let words = vec!["пятак", "пятТка", "тяпка", "листок", "слиток", "столик", "слиток", "Пятак"];
    let anagram_map = find_anagrams(&words);

    for (key, group) in anagram_map {
        println!("{}: {:?}", key, group);
    }
}
