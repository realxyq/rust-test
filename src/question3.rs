// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。


pub fn third_question() {
    println!("Input String: ");
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).unwrap();

    let words: Vec<&str> = input_str.split_whitespace().collect();
    let mut word_count = std::collections::HashMap::new();
    for word in words {
        *word_count.entry(word).or_insert(0) += 1;
    }

    let mut sorted_words: Vec<_> = word_count.iter().collect();
    sorted_words.sort_by(|(a_word, a_count), (b_word, b_count)|{
        b_count.cmp(a_count).then(a_word.cmp(b_word))
    });

    for (word, count) in sorted_words.iter().rev() {
        println!("{}: {}", word, count);
    }
}

mod tests {
    use std::io::Read;
    #[test]
    fn test() {
        let input_str = "apple banana pear banana apple banana";
        let expected_output = "pear: 1\nbanana: 2\napple: 2\n";
        let mut output = String::new();
        super::third_question();
        std::io::stdin().read_to_string(&mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}