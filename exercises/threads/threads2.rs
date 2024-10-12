// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// ~I AM NOT DONE

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

struct JobStatus {
    jobs_completed: u32,
}
//? 多进程访问必须要使用Mutex  Arc
//? Arc 允许多个线程安全地共享对同一个 Mutex 的所有权，
//? Mutex 则确保在任意时刻只有一个线程可以修改 T 的内容。
fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        //? status.lock()：尝试获取 Mutex 的锁。
        //? unwrap()：处理 Result，如果获取锁失败，程序将 panic（崩溃）。
        //? jobs_completed：一旦成功获得锁，就可以安全地访问和读取 jobs_completed 的值。
        println!("jobs completed {}",status.lock().unwrap().jobs_completed);

    }
}
