// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.
//变量遮蔽
//Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的，如下所示：
// ~I AM NOT DONE

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
