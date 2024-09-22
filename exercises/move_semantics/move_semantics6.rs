// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.
//! rust中 只读--> 引用。修改-->所有权
// ~I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership 原因：此函数仅需要读取 data 的最后一个字符，并不需要修改或持久化 data。
//不给予所有权的可以避免更改数据
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
//此函数需要将整个字符串转换为大写，这涉及到对 data 的修改，因此需要获取所有权。
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
