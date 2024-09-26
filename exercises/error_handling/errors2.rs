// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

// ~I AM NOT DONE
//? 在 Rust 中，Err(e) 中的 e 是一个变量名，用来捕获 Err 情况下的错误值。
//? 这个变量名可以是任意合法的标识符，不仅仅限于 e。实际上，你可以使用任何你喜欢的名字来代替 e。
use std::num::ParseIntError;

//?parse::<i32>() 方法会返回一个 Result 类型的值，这个 Result 要么是 Ok(T)，要么是 Err(T)。
//? 使用let match 会把值绑定在qty上，成功则为Ok（T)，不成功则为Err（s）
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let qty = match item_quantity.parse::<i32>(){
        Ok(num)=>num,
        Err(s)=>return Err(s), 
    };
    let processing_fee = 1;
    let cost_per_item = 5;
    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
