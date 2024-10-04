use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前的时间戳（以UNIX纪元为基准的秒数）
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("时间倒流了")
        .as_secs();

    // 设置 TEST_FOO 环境变量
    println!("cargo:rerun-if-changed=build.rs"); // 如果 build.rs 发生变化，重新运行构建脚本
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 基于当前时间设置特性标志
    if timestamp % 2 == 0 {
        // 如果时间戳是偶数，启用 `pass` 特性
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}
