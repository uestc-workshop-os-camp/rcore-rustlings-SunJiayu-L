// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// ~I AM NOT DONE
// 同一作用域，特定数据只能有一个可变引用
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;   
    // 这里不会报错，因为编译器分析出来y的作用域在这里结束，所以不出现两个可变引用冲突
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}




