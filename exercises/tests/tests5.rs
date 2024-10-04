// tests5.rs
//

//当在一个项目声明上标记了`unsafe`时，比如一个函数、一个trait等等，它会在旁边声明一个contract。
//但是，合同的内容不能只用一个关键字来表达。因此，你有责任在文档注释的`# Safety`部分手动说明它。
//当在大括号括起来的代码块上标记`unsafe`时，它声明遵守了某种约定，例如某个指针参数的有效性，某个内存地址的所有权。
//然而，像上面的文本一样，你仍然需要在代码块的注释中说明如何观察契约。
//注意:所有的注释都是为了代码的可读性和可维护性，而Rust编译器将其对代码可靠性的信任交给了自己!
//如果你不能证明自己代码的内存安全性和可靠性，请退一步，使用安全的代码!
// An `unsafe` in Rust serves as a contract.
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

// ~I AM NOT DONE
//? 测试目的，有一个u32的变量t,初始值为0x12345678，通过调用modify_by_address函数，将t的值修改为0xAABBCCDD。
//? 通过 &mut t as *mut u32 as usize 将 t 的可变引用转换为一个原始指针，然后再转换为 usize。这表示 t 的内存地址。
//? 在解引用的时候要正确转换回指针类型，然后才能进行解引用和修改。
/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
