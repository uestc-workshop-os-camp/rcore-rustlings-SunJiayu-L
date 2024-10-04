// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// ~I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);

    // 使用模式匹配来处理 Some 和 None
    match option {
        Some(x) => res += x,
        None => println!("Option is None"),
    }

    println!("{}", res);
}
