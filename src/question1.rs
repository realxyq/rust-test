// 从命令行读取一个整数 n（若读取失败或没有输入则默认 n = 5）。
// 打印从 1 到 n 的所有整数，每行一个。
// 若该整数可以被 3 整除，则在数字后面附加输出 "Fizz"；若可以被 5 整除，则附加输出 "Buzz"；若同时满足可以被 3 和 5 整除的情况，则输出 "FizzBuzz"。
pub fn first_question() {
    let mut n = 5;
    let mut input = String::new();
    println!("Input number :");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    if let Ok(num) = input.parse::<i32>() {
        n = num;
    }
    println!("Final output :");
    for i in 1..=n {
        let mut output = String::new();
        if i % 3 == 0 {
            output.push_str("Fizz");
        }
        if i % 5 == 0 {
            output.push_str("Buzz");
        }
        if output.is_empty() {
            output.push_str(&i.to_string());
        }
        println!("{}", output);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_question() {
        first_question();
    }
}

