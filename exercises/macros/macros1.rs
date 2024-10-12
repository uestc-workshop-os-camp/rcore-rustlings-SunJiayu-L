// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

// ~I AM NOT DONE

//! 这个Rust宏定义了一个名为my_macro的宏，当以my_macro!()形式调用时，会输出字符串"Check out my macro!"。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
