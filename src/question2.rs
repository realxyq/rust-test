// 定义一个 Student 结构体，包含以下字段：name、age、score
// 实现以下功能：
// - new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
// - show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
// - is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。
pub struct Student {
    name: String,
    age: u8,
    score: f32,
}
impl Student {
    pub fn new(name: &str, age: u8, score: f32) -> Self {
        Self {
            name: name.to_string(),
            age,
            score,
        }
    }

    pub fn show(&self) {
        println!("Name: {}, Age: {}, Score: {:.1}", self.name, self.age, self.score);
    }

    pub fn is_passed(&self) -> bool {
        self.score >= 60.0
    }
}


pub fn second_quetion() {
    let alice = Student::new("Alice", 18, 95.5);
    alice.show();
    assert!(alice.is_passed());
}

mod tests {
    use crate::question2::Student;
    #[test]
    fn test() {
        let alice = Student::new("Alice", 18, 95.5);
        assert_eq!("Alice", alice.name);
        assert_eq!(18, alice.age);
        assert_eq!(95.5, alice.score);
    }
}
