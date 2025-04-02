mod question1;
mod question2;
mod question3;
mod question4;
mod question5;
mod question6;

fn main() {
    println!("Hello, world!");
    //question1::first_question();
    //question2::second_question();
    //question3::third_question();

    println!("Input String: ");
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).unwrap();
    question4::fourth_question(input_str.trim());

    //question5::fifth_question();
    //question6::sixth_question();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(1 + 1, 2);
    }
}

