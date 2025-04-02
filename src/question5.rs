// 使用多线程并行计算某个函数的值或模拟并发任务。
// 需要创建 3 个线程同时进行下载，并在下载完成后将结果（例如“URL + 下载完成”）
// 通过消息通道（std::sync::mpsc）发送回主线程。主线程依次接收并打印结果。
use std::process::Command;
use std::sync::mpsc;
use std::thread;

fn scp_transfer(source: &str, destination: &str) -> String {
    let status = Command::new("scp")
        .arg(source)
        .arg(destination)
        .status()
        .expect("Failed to execute scp command");

    if status.success() {
        format!("SCP {} -> {} download success", source, destination)
    } else {
        format!("SCP {} -> {} download failed", source, destination)
    }
}

fn main() {
    let tasks = vec![
        ("/path/to/local/file1.txt", "/path/to/destination/file1.txt"),
        ("/path/to/local/file2.log", "/path/to/destination/file2.log"),
        ("/path/to/local/file3.dat", "/path/to/destination/file3.dat")
    ];

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for (src, dst) in tasks {
        let tx = tx.clone();
        let src = src.to_string();
        let dst = dst.to_string();

        let handle = thread::spawn(move || {
            let result = scp_transfer(&src, &dst);
            tx.send(result).expect("send failed");
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("failed to join thread");
    }

    println!("results:");
    for _ in 0..tasks.len() {
        let received = rx.recv().expect("get result failed");
        println!("{}", received);
    }
}