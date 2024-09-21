// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

// ~I AM NOT DONE

//在 Rust 中，当你传递一个 Vec<i32> 给一个函数时，你实际上是在传递整个向量的所有权，而不是仅仅传递第一个元素的地址。
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {   //v.iter_mut() 返回一个可变迭代器，每个元素都是一个 &mut i32 引用。
                                    //遍历每个元素，element 是一个 &mut i32 引用。
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2;
    }  
    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {   //v.iter()：获取 v 的迭代器。使用 map 方法 .collect()：将转换后的迭代器收集成一个新的向量 Vec<i32>。
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
