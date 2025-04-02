// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。
use std::io::Read;
use std::io::Write;
pub fn fourth_question(input: &str) {
    let mut file = match std::fs::File::open(input) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut char_count = 0;
    let mut line_count = 0;
    for c in contents.chars() {
        if c == '\n' {
            line_count += 1;
        } else {
            char_count += 1;
        }
    }

    let mut output_file = match std::fs::OpenOptions::new()//创建或覆盖output.txt
        .create(true)
        .write(true)
        .open("output.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match writeln!(
        output_file,
        "Character count: {}\nLine count: {}",
        char_count,
        line_count
    ) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::fourth_question;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    // 辅助函数：创建测试文件
    fn create_test_file(path: &str, content: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    // 辅助函数：读取输出文件内容
    fn read_output_file() -> String {
        fs::read_to_string("output.txt").unwrap_or_default()
    }

    // 在每个测试前清理可能存在的输出文件
    fn setup() {
        let _ = fs::remove_file("output.txt");
    }

    #[test]
    fn test_normal_file() {
        setup();
        create_test_file("test_input.txt", "Hello\nWorld\nRust");
        
        fourth_question("test_input.txt");
        
        let output = read_output_file();
        assert!(output.contains("Character count: 13"));
        assert!(output.contains("Line count: 2"));

        fs::remove_file("test_input.txt").unwrap();
    }

    #[test]
    fn test_empty_file() {
        setup();
        create_test_file("empty_input.txt", "");
        
        fourth_question("empty_input.txt");
        
        let output = read_output_file();
        assert!(output.contains("Character count: 0"));
        assert!(output.contains("Line count: 0"));
        
        fs::remove_file("empty_input.txt").unwrap();
    }

    #[test]
    fn test_file_with_only_newlines() {
        setup();
        create_test_file("newlines.txt", "\n\n\n");
        
        fourth_question("newlines.txt");
        
        let output = read_output_file();
        assert!(output.contains("Character count: 0"));
        assert!(output.contains("Line count: 3"));
        
        fs::remove_file("newlines.txt").unwrap();
    }

    #[test]
    fn test_nonexistent_file() {
        setup();
        let result = std::panic::catch_unwind(|| {
            fourth_question("nonexistent.txt");
        });
        
        assert!(result.is_err());
        
        assert!(!Path::new("output.txt").exists());
    }

    #[test]
    fn test_output_file_overwrite() {
        setup();
        create_test_file("output.txt", "旧内容");

        create_test_file("overwrite_input.txt", "测试内容");
        
        fourth_question("overwrite_input.txt");
        
        let output = read_output_file();
        assert!(!output.contains("旧内容"));
        assert!(output.contains("Character count: 12"));
        assert!(output.contains("Line count: 1"));
        
        fs::remove_file("overwrite_input.txt").unwrap();
    }
}