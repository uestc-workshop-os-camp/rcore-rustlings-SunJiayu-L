// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// ~I AM NOT DONE

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        //? 不是if let 条件判断 更像是if （）
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        //? 定义一个动态数组叫optional_integers 动态数组的元素类型是 Option<i8>，表示每个元素可能是一个整数（Some(i8)）或没有值（None）。
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        //?pop返回一个Some（元素） 而optional_integers元素为Option<i8>。
        //? 所以最后返回为如果向量中还有元素，pop() 会返回 Some(Some(value))，其中 value 是向量最后一个元素的值。

        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }
        assert_eq!(cursor, 0);
    }
}
